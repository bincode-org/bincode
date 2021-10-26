#![cfg(feature = "derive")]

/// HID-IO Packet Buffer Struct
///
/// # Remarks
/// Used to store HID-IO data chunks. Will be chunked into individual packets on transmission.
#[repr(C)]
#[derive(PartialEq, Clone, Debug, bincode::Encode)]
#[cfg_attr(feature = "defmt-impl", derive(defmt::Format))]
pub struct HidIoPacketBuffer<const H: usize> {
    /// Type of packet (Continued is automatically set if needed)
    pub ptype: HidIoPacketType,
    /// Packet Id
    pub id: HidIoCommandId,
    /// Packet length for serialization (in bytes)
    pub max_len: u32,
    /// Payload data, chunking is done automatically by serializer
    pub data: Vec<u8, H>,
    /// Set False if buffer is not complete, True if it is
    pub done: bool,
}
