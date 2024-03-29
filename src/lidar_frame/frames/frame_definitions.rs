use super::*;

#[derive(Len, Debug, Deserialize)]
pub struct CartesianPoint {
    x: i32, // millimeters
    y: i32,
    z: i32,
    reflectivity: u8,
    tag: u8,
}

#[derive(Len, Debug, Deserialize)]
pub struct SphericalPoint {
    depth: u32,  // millimeters
    zenith: u16, // 0.01 degree
    azimuth: u16,
    reflectivity: u8,
    tag: u8,
}

#[derive(Len, Debug, Deserialize)]
pub struct DataFrame {
    version: u8,
    slot_id: u8,
    lidar_id: u8,
    reserved: u8,
    status_code: u32,
    timestamp_type: u8,
    data_type: u8,
    timestamp: u64,
}

/// Command set and command id.
#[derive(Debug, Serialize, Deserialize, Len, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Cmd {
    cmd_set: u8,
    cmd_id: u8,
}

/// Broadcast frame, received from lidar
#[derive(Debug, Serialize, Deserialize, Len, GetCmd)]
pub struct Broadcast {
    cmd: Cmd,
    broadcast_code: [u8; 16],
    dev_type: u8,
    _reserved: u16,
}

/// Handshake to connect lidar, ip address and ports is constantly configured in cfg.rs
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct HandshakeReq {
    cmd: Cmd,
    user_ip: [u8; 4],
    data_port: u16,
    cmd_port: u16,
    imu_port: u16,
}

/// Handshake to connect lidar
pub const HANDSHAKE_REQ: HandshakeReq = HandshakeReq {
    cmd: Cmd {
        cmd_set: 0x00,
        cmd_id: 0x01,
    },
    user_ip: USER_IP,
    data_port: DATA_PORT,
    cmd_port: CMD_PORT,
    imu_port: IMU_PORT,
};

#[derive(Debug, Deserialize, CheckStatus)]
pub struct CommonResp {
    ret_code: u8,
}

/// Request device information
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct DeviceInfoReq(Cmd);

/// Request device information
pub const DEVICE_INFO_REQ: DeviceInfoReq = DeviceInfoReq(Cmd {
    cmd_set: 0x00,
    cmd_id: 0x02,
});

#[derive(Debug, Deserialize, CheckStatus)]
pub struct DeviceInfoResp {
    ret_code: u8,
    version: [u8; 4],
}

/// Send Heartbeat frame to lidar
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct HeartbeatReq(Cmd);

/// Send Heartbeat frame to lidar
pub const HEARTBEAT_REQ: HeartbeatReq = HeartbeatReq(Cmd {
    cmd_set: 0x00,
    cmd_id: 0x03,
});

#[derive(Debug, Deserialize, CheckStatus)]
pub struct HeartbeatResp {
    ret_code: u8,
    work_state: u8,
    feature_msg: u8,
    ack_msg: u32,
}

/// Start or end lidar sample, 0x00: start, 0x01: end
#[derive(Debug, Serialize, Deserialize, Len, GetCmd)]
pub struct SampleCtrlReq {
    cmd: Cmd,
    sample_ctrl: u8,
}

/// Start lidar sampling
pub const SAMPLE_START_REQ: SampleCtrlReq = SampleCtrlReq {
    cmd: Cmd {
        cmd_set: 0x00,
        cmd_id: 0x04,
    },
    sample_ctrl: 0x01,
};

/// End lidar sampling
pub const SAMPLE_END_REQ: SampleCtrlReq = SampleCtrlReq {
    cmd: Cmd {
        cmd_set: 0x00,
        cmd_id: 0x04,
    },
    sample_ctrl: 0x00,
};

/// Change point cloud coordinate type, 0x00: Cartesian, 0x01: Spherical
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct ChangeCoordinateReq {
    cmd: Cmd,
    coordinate_type: u8,
}

/// Change point cloud coordinate type to cartesian
pub const CARTESIAN_COORDINATE_REQ: ChangeCoordinateReq = ChangeCoordinateReq {
    cmd: Cmd {
        cmd_set: 0x00,
        cmd_id: 0x05,
    },
    coordinate_type: 0x00,
};

/// Change point cloud coordinate type to spherical
pub const SPHERICAL_COORDINATE_REQ: ChangeCoordinateReq = ChangeCoordinateReq {
    cmd: Cmd {
        cmd_set: 0x00,
        cmd_id: 0x05,
    },
    coordinate_type: 0x01,
};

/// Disconnect from lidar
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct DisconnectReq(Cmd);

/// Disconnect from lidar
pub const DISCONNECT_REQ: DisconnectReq = DisconnectReq(Cmd {
    cmd_set: 0x00,
    cmd_id: 0x06,
});

/// Configure ip address, net mask and gateway address
#[derive(Debug, Serialize, Deserialize, Len, GetCmd)]
pub struct IpConfigReq {
    cmd: Cmd,
    ip_mode: u8,
    ip_addr: [u8; 4],
    net_mask: [u8; 4],
    gw_addr: [u8; 4],
}

impl IpConfigReq {
    pub fn new(ip_mode: u8, ip_addr: [u8; 4], net_mask: [u8; 4], gw_addr: [u8; 4]) -> Self {
        IpConfigReq {
            cmd: Cmd {
                cmd_set: 0x00,
                cmd_id: 0x08,
            },
            ip_mode,
            ip_addr,
            net_mask,
            gw_addr,
        }
    }
}

/// Get ip info of device
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct IpInfoReq(Cmd);

/// Get ip information of device
pub const IP_INFO_REQ: IpInfoReq = IpInfoReq(Cmd {
    cmd_set: 0x00,
    cmd_id: 0x09,
});

/// Reboot device
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct RebootReq {
    cmd: Cmd,
    timeout: u16,
}

impl RebootReq {
    pub fn new(timeout: u16) -> Self {
        RebootReq {
            cmd: Cmd {
                cmd_set: 0x00,
                cmd_id: 0x0A,
            },
            timeout,
        }
    }
}

/// Reboot device immediately
pub const REBOOT_IMMEDIATE_REQ: RebootReq = RebootReq {
    cmd: Cmd {
        cmd_set: 0x00,
        cmd_id: 0x0A,
    },
    timeout: 0x00,
};

/// Set flash configuration, won't lose after reboot
#[derive(Debug)]
pub struct WriteFlashReq {
    cmd: Cmd,
    high_sensitivity: bool,
    repetitive_scan: bool,
    slot_id: u8,
}

impl Serialize for WriteFlashReq {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serializer = serializer.serialize_tuple_struct("WriteFlashReq", 17)?;
        serializer.serialize_field(&self.cmd)?;
        serializer.serialize_field(&0x01u16)?;
        serializer.serialize_field(&0x01u16)?;
        serializer.serialize_field(&self.high_sensitivity)?;
        serializer.serialize_field(&0x02u16)?;
        serializer.serialize_field(&0x01u16)?;
        serializer.serialize_field(&self.repetitive_scan)?;
        serializer.serialize_field(&0x03u16)?;
        serializer.serialize_field(&0x01u16)?;
        serializer.serialize_field(&self.slot_id)?;
        serializer.end()
    }
}

impl WriteFlashReq {
    pub fn new(high_sensitivity: bool, repetitive_scan: bool, slot_id: u8) -> Self {
        match slot_id {
            0x01u8..=0x09u8 => WriteFlashReq {
                cmd: Cmd {
                    cmd_set: 0x00,
                    cmd_id: 0x0B,
                },
                high_sensitivity,
                repetitive_scan,
                slot_id,
            },
            _ => panic!("Invalid slot_id: {}", slot_id),
        }
    }
}

/// Set default flash configuration
pub const WRITE_FLASH_DEFAULT_REQ: WriteFlashReq = WriteFlashReq {
    cmd: Cmd {
        cmd_set: 0x00,
        cmd_id: 0x0B,
    },
    high_sensitivity: true,
    repetitive_scan: false,
    slot_id: 0x01,
};

impl Len for WriteFlashReq {
    fn len() -> u16 {
        (mem::size_of::<u8>() * 3 + mem::size_of::<u16>() * 2 * 3) as u16 + Cmd::len()
    }
}

/// Set Lidar mode
/// 0x01: Normal mode
/// 0x02: Low power mode
/// 0x03: Standby mode
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct ModeSwitchReq {
    cmd: Cmd,
    mode: u8,
}

impl ModeSwitchReq {
    pub fn new(mode: u8) -> Self {
        match mode {
            0x01u8..=0x03u8 => ModeSwitchReq {
                cmd: Cmd {
                    cmd_set: 0x01,
                    cmd_id: 0x00,
                },
                mode,
            },
            _ => panic!("Invalid lidar mode: {}", mode),
        }
    }
}

/// Write outer param
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct WriteOuterParameters {
    cmd: Cmd,
    roll: f32,
    pitch: f32,
    yaw: f32,
    x: i32,
    y: i32,
    z: i32,
}

impl WriteOuterParameters {
    pub fn new(roll: f32, pitch: f32, yaw: f32, x: i32, y: i32, z: i32) -> Self {
        WriteOuterParameters {
            cmd: Cmd {
                cmd_set: 0x01,
                cmd_id: 0x01,
            },
            roll,
            pitch,
            yaw,
            x,
            y,
            z,
        }
    }
}

/// Get outer parameters
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct ReadOuterParameters(Cmd);

/// Read outer parameters of lidar
pub const READ_OUTER_PARAMETERS: ReadOuterParameters = ReadOuterParameters(Cmd {
    cmd_set: 0x01,
    cmd_id: 0x02,
});

/// Set Lidar Return Mode:
/// 0x00: Single Return First
/// 0x01: Single Return Strongest
/// 0x02: Dual Return
/// 0x03: Triple Return
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct SetReturnMode {
    cmd: Cmd,
    mode: u8,
}

impl SetReturnMode {
    pub fn new(mode: u8) -> Self {
        match mode {
            0x00u8..=0x03u8 => SetReturnMode {
                cmd: Cmd {
                    cmd_set: 0x01,
                    cmd_id: 0x06,
                },
                mode,
            },
            _ => panic!("Invalid return mode: {}", mode),
        }
    }
}

/// Get Lidar Return Mode
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct GetReturnMode(Cmd);

pub const GET_RETURN_MODE: GetReturnMode = GetReturnMode(Cmd {
    cmd_set: 0x01,
    cmd_id: 0x07,
});

/// Update UTC Synchronize Time
#[derive(Debug, Serialize, Len, GetCmd)]
pub struct UpdateUtcSyncTime {
    cmd: Cmd,
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    microsecond: u32,
}

impl UpdateUtcSyncTime {
    pub fn new(year: u8, month: u8, day: u8, hour: u8, microsecond: u32) -> Self {
        UpdateUtcSyncTime {
            cmd: Cmd {
                cmd_set: 0x01,
                cmd_id: 0x0A,
            },
            year,
            month,
            day,
            hour,
            microsecond,
        }
    }
}

#[derive(Debug)]
pub struct ControlFrame<'a, T> {
    seq_num: u16,
    frame_seg: &'a T,
}

impl<'a, T> ControlFrame<'a, T> {
    pub fn new(seq_num: u16, frame_seg: &'a T) -> Self {
        ControlFrame { seq_num, frame_seg }
    }

    pub fn serialize(&self) -> Result<Vec<u8>>
    where
        T: Serialize + Len,
    {
        let crc16 = Crc::<u16>::new(&CRC_16_MCRF4XX);
        let mut digest16 = crc16.digest_with_initial(CRC16_INIT);

        let mut digest32 = crc32fast::Hasher::new_with_initial(CRC32_INIT);

        let buffer_len = Self::len();

        let mut buf = Vec::with_capacity(buffer_len as usize);

        // sof
        buf.push(0xAAu8);

        // version of communication protocol
        buf.push(0x01u8);

        // length of data frame
        buf.extend((buffer_len).to_le_bytes());

        // command type, always CMD: 0x00
        buf.push(0x00u8);

        buf.extend(self.seq_num.to_le_bytes());

        // calculate CRC16
        digest16.update(&buf);
        buf.extend(digest16.finalize().to_le_bytes());

        // serialize data segment
        bincode::serialize_into(&mut buf, &self.frame_seg)?;

        // calculate CRC32
        digest32.update(&buf);

        buf.extend(digest32.finalize().to_le_bytes());

        Ok(buf)
    }
}

impl<T> Len for ControlFrame<'_, T>
where
    T: Len,
{
    fn len() -> u16 {
        const SOF_LEN: usize = mem::size_of::<u8>();
        const VERSION_LEN: usize = mem::size_of::<u8>();
        const LENGTH_LEN: usize = mem::size_of::<u16>();
        const CMD_TYPE_LEN: usize = mem::size_of::<u8>();
        const SEQ_NUM_LEN: usize = mem::size_of::<u16>();
        const CRC16_LEN: usize = mem::size_of::<u16>();
        const CRC32_LEN: usize = mem::size_of::<u32>();
        (SOF_LEN + VERSION_LEN + LENGTH_LEN + CMD_TYPE_LEN + SEQ_NUM_LEN + CRC16_LEN + CRC32_LEN)
            as u16
            + T::len()
    }
}

/// deserialize from buffer, return tuple of sequence number and inner frame
pub fn deserialize_resp<'a>(buffer: &'a [u8]) -> Result<(u16, Cmd, &'a [u8])> {
    let len = u16::from_le_bytes(buffer[2..=3].try_into()?) as usize;
    if buffer.len() != len {
        return Err(anyhow!(
            concat!(
                "Cannot deserialize the serial due to an incompatible length:",
                "the length of the serial is {}, ",
                "while the length of the <ControlFrame> frame is {}."
            ),
            buffer.len(),
            len,
        ));
    }

    let crc16 = Crc::<u16>::new(&CRC_16_MCRF4XX);
    let mut digest16 = crc16.digest_with_initial(CRC16_INIT);
    digest16.update(&buffer[..7]);
    let checksum_recv = u16::from_le_bytes(buffer[7..=8].try_into()?);
    let checksum_cal = digest16.finalize();
    if checksum_cal != checksum_recv {
        return Err(anyhow!(
            concat!(
                "Crc16 for header of <ControlFrame> failed",
                "checksum received is 0x{:X?}, ",
                "while the calculated checksum is 0x{:X?}.",
            ),
            checksum_recv,
            checksum_cal
        ));
    }

    let mut digest32 = crc32fast::Hasher::new_with_initial(CRC32_INIT);
    digest32.update(&buffer[..len - 4]);

    let checksum_recv = u32::from_le_bytes(buffer[len - 4..].try_into()?);
    let checksum_cal = digest32.finalize();

    if checksum_cal != checksum_recv {
        return Err(anyhow!(
            concat!(
                "Crc32 for frame of <ControlFrame> failed",
                "checksum received is {:X?}, ",
                "while the calculated checksum is {:X?}.",
            ),
            checksum_recv,
            checksum_cal
        ));
    }

    let seq_num = u16::from_le_bytes(buffer[5..=6].try_into()?);

    bincode::deserialize(&buffer[9..11])
        .map_err(|e| anyhow!("Failed to deserialize data segment: {}", e))
        .map(|cmd| (seq_num, cmd, &buffer[11..len - 4]))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serialize() {
        let control_frame = ControlFrame::new(0x11, &HANDSHAKE_REQ);
        let serialized = control_frame.serialize().unwrap();

        assert_eq!(
            serialized,
            vec![
                170, 1, 27, 0, 0, 17, 0, 29, 194, 0, 1, 50, 1, 168, 192, 80, 195, 81, 195, 82, 195,
                34, 129, 121, 236
            ]
        );
    }
}
// #[test]
// fn test_deserialize() {
//     let mut control_frame = ControlFrame::new(CmdType::Cmd, 0x0, Broadcast::new());
//     let serial = vec![
//         0xAA, 0x1, 0x22, 0x0, 0x0, 0x11, 0x0, 0xA8, 0x47, 0x0, 0x0, 0x1, 0x1, 0x1, 0x1, 0x1,
//         0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x0, 0x45, 0x11, 0x9E, 0xBE,
//         0x83, 0x49,
//     ];
//     control_frame.deserialize(&serial).unwrap();
//     assert_eq!(control_frame.sof, 0xAA);
//     assert_eq!(control_frame.version, 0x1);
//     assert_eq!(control_frame.cmd_type, CmdType::Cmd);
//     assert_eq!(control_frame.seq_num, 0x11);
// }
// }
