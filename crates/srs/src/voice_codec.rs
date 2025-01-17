use std::io::{self, Cursor, Read, Write};

// use byteorder::ByteOrder;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::{BufMut, BytesMut};
use tokio_codec::{Decoder, Encoder, LengthDelimitedCodec};

pub struct VoiceCodec {
    inner: LengthDelimitedCodec,
    is_head: bool,
}

impl VoiceCodec {
    pub fn new() -> Self {
        VoiceCodec {
            inner: LengthDelimitedCodec::builder()
                .length_field_offset(0)
                .length_field_length(2)
                .length_adjustment(-2)
                .little_endian()
                .new_codec(),
            is_head: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Modulation {
    AM,
    FM,
    Intercom,
    Disabled,
}

#[derive(Debug, Clone)]
pub enum Encryption {
    None,
    JustOverlay,
    Full,
    CockpitToggleOverlayCode,
}

#[derive(Debug, Clone)]
pub struct Frequency {
    pub freq: f64,
    pub modulation: Modulation,
    pub encryption: Encryption,
}

#[derive(Debug)]
pub enum Packet {
    Ping([u8; 22]),
    Voice(VoicePacket),
}

#[derive(Debug)]
pub struct VoicePacket {
    // TODO: use Bytes instead?
    pub audio_part: Vec<u8>,
    pub frequencies: Vec<Frequency>,
    pub unit_id: u32,
    pub packet_id: u64,
    pub sguid: [u8; 22],
}

impl Decoder for VoiceCodec {
    // UdpFramed, what VoiceCodec is used with, has a strange behavior in Tokio currently. If the
    // codec would return `None`, which is actually an indication for that the voice codec needs
    // more data to produce a valid item, the UdpFramed would yield the `None` as well. Though,
    // a `None` from a stream means the stream is closed. This is planned to be fixed in tokio
    // 0.2.0. Until then, we are using an option item here instead, so the stream would return
    // `Some(None)` instead.
    type Item = Option<VoicePacket>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // discard ping messages
        if self.is_head && buf.len() == 22 {
            return Ok(None);
        }

        if let Some(bytes) = self.inner.decode(buf)? {
            self.is_head = true;

            let len = bytes.len() as u64;
            let mut rd = Cursor::new(bytes);

            let len_audio_part = rd.read_u16::<LittleEndian>()? as u64;
            let len_frequencies = rd.read_u16::<LittleEndian>()? as u64;

            assert_eq!(
                len,
                4 + len_audio_part as u64 + len_frequencies + 4 + 8 + 22
            );

            let mut audio_part = vec![0u8; len_audio_part as usize];
            rd.read_exact(&mut audio_part)?;

            let len_before = rd.position();
            let mut frequencies = Vec::new();
            while rd.position() - len_before < len_frequencies {
                let freq = rd.read_f64::<LittleEndian>()?;
                let modulation = match rd.read_u8()? {
                    0 => Modulation::AM,
                    1 => Modulation::FM,
                    2 => Modulation::Intercom,
                    3 => Modulation::Disabled,
                    _ => Modulation::AM,
                };
                let encryption = match rd.read_u8()? {
                    0 => Encryption::None,
                    1 => Encryption::JustOverlay,
                    2 => Encryption::Full,
                    3 => Encryption::CockpitToggleOverlayCode,
                    _ => Encryption::None,
                };
                frequencies.push(Frequency {
                    freq,
                    modulation,
                    encryption,
                });
            }

            let unit_id = rd.read_u32::<LittleEndian>()?;
            let packet_id = rd.read_u64::<LittleEndian>()?;

            let mut sguid = [0; 22];
            rd.read_exact(&mut sguid)?;

            assert_eq!(rd.position(), len);

            Ok(Some(Some(VoicePacket {
                audio_part,
                frequencies,
                unit_id,
                packet_id,
                sguid,
            })))
        } else {
            self.is_head = false;
            Ok(Some(None))
        }
    }
}

impl Encoder for VoiceCodec {
    type Item = Packet;
    type Error = io::Error;

    fn encode(&mut self, packet: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let packet = match packet {
            Packet::Ping(sguid) => {
                buf.put_slice(&sguid);
                return Ok(());
            }
            Packet::Voice(packet) => packet,
        };

        let capacity = 4 + packet.audio_part.len() + packet.frequencies.len() * 10 + 4 + 8 + 22;
        let mut wd = Cursor::new(Vec::with_capacity(capacity));

        // header segment will be written at the end
        wd.set_position(4);

        // - AUDIO SEGMENT
        let len_before = wd.position();
        wd.write_all(&packet.audio_part)?;
        let len_audio_part = wd.position() - len_before;

        // - FREQUENCY SEGMENT
        let len_before = wd.position();
        for f in packet.frequencies {
            wd.write_f64::<LittleEndian>(f.freq)?;

            wd.write_u8(match f.modulation {
                Modulation::AM => 0,
                Modulation::FM => 1,
                Modulation::Intercom => 2,
                Modulation::Disabled => 3,
            })?;
            wd.write_u8(match f.encryption {
                Encryption::None => 0,
                Encryption::JustOverlay => 1,
                Encryption::Full => 2,
                Encryption::CockpitToggleOverlayCode => 3,
            })?;
        }

        let len_frequency = wd.position() - len_before;

        // - FIXED SEGMENT
        wd.write_u32::<LittleEndian>(packet.unit_id)?;
        wd.write_u64::<LittleEndian>(packet.packet_id)?;
        wd.write_all(&packet.sguid)?;

        // - HEADER SEGMENT
        wd.set_position(0);

        // Packet Length:
        // the final packet will start with the total packet length, but this will be added by
        // the inner fixed codec

        // AudioPart1 Length
        wd.write_u16::<LittleEndian>(len_audio_part as u16)?;
        // FrequencyPart Length
        wd.write_u16::<LittleEndian>(len_frequency as u16)?;

        let frame = wd.into_inner();
        assert_eq!(frame.len(), capacity);

        self.inner.encode(frame.into(), buf)
    }
}

impl From<VoicePacket> for Packet {
    fn from(p: VoicePacket) -> Self {
        Packet::Voice(p)
    }
}
