#![allow(non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use] extern crate bitflags;
extern crate simple_error;

use simple_error::SimpleError;

#[derive(Debug)]
pub struct Error {
    pub result: HRESULT,
}

pub struct Device {
    implementation: *mut IDeckLink,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            decklink_release(self.implementation);
        }
    }
}

impl REFIID {
    fn new(b: [u8; 16]) -> REFIID {
        REFIID{
            byte0: b[0],
            byte1: b[1],
            byte2: b[2],
            byte3: b[3],
            byte4: b[4],
            byte5: b[5],
            byte6: b[6],
            byte7: b[7],
            byte8: b[8],
            byte9: b[9],
            byte10: b[10],
            byte11: b[11],
            byte12: b[12],
            byte13: b[13],
            byte14: b[14],
            byte15: b[15],
        }
    }
}

impl Device {
    pub fn get_model_name(&self) -> Result<String, Error> {
        unsafe {
            let mut buf: *mut Buffer = std::ptr::null_mut();
            match decklink_get_model_name(self.implementation, &mut buf) {
                0 => {
                    let ret = std::ffi::CStr::from_ptr(buffer_data(buf) as *const i8).to_str().unwrap_or("").to_string();
                    buffer_release(buf);
                    Ok(ret)
                },
                result => Err(Error{
                    result: result,
                }),
            }
        }
    }

    fn query_interface<T>(&self, iid: REFIID) -> Result<*mut T, Error> {
        unsafe {
            let mut iface: *mut T = std::ptr::null_mut();
            match decklink_query_interface(self.implementation, iid, std::mem::transmute::<&mut *mut T, &mut *mut std::ffi::c_void>(&mut iface)) {
                0 => Ok(iface),
                result => Err(Error{
                    result: result,
                }),
            }
        }
    }

    pub fn query_attributes(&self) -> Result<Attributes, Error> {
        match self.query_interface(REFIID::new([0xAB,0xC1,0x18,0x43,0xD9,0x66,0x44,0xCB,0x96,0xE2,0xA1,0xCB,0x5D,0x31,0x35,0xC4])) {
            Ok(iface) => Ok(Attributes{
                implementation: iface
            }),
            Err(e) => Err(e),
        }
    }

    pub fn query_status(&self) -> Result<Status, Error> {
        match self.query_interface(REFIID::new([0x5F,0x55,0x82,0x00,0x40,0x28,0x49,0xBC,0xBE,0xAC,0xDB,0x3F,0xA4,0xA9,0x6E,0x46])) {
            Ok(iface) => Ok(Status{
                implementation: iface
            }),
            Err(e) => Err(e),
        }
    }

    pub fn query_input(&self) -> Result<Input, Error> {
        match self.query_interface(REFIID::new([0xAF,0x22,0x76,0x2B,0xDF,0xAC,0x48,0x46,0xAA,0x79,0xFA,0x88,0x83,0x56,0x09,0x95])) {
            Ok(iface) => Ok(Input{
                implementation: iface
            }),
            Err(e) => Err(e),
        }
    }

    pub fn query_output(&self) -> Result<Output, Error> {
        match self.query_interface(REFIID::new([0xCC,0x5C,0x8A,0x6E,0x3F,0x2F,0x4B,0x3A,0x87,0xEA,0xFD,0x78,0xAF,0x30,0x05,0x64])) {
            Ok(iface) => Ok(Output{
                implementation: iface
            }),
            Err(e) => Err(e),
        }
    }
}

pub struct Attributes {
    implementation: *mut IDeckLinkAttributes,
}

bitflags! {
    pub struct VideoIOSupport: u32 {
        const CAPTURE = _BMDVideoIOSupport_bmdDeviceSupportsCapture;
        const PLAYBACK = _BMDVideoIOSupport_bmdDeviceSupportsPlayback;
    }
}

impl Attributes {
    fn get_flag(&self, id: BMDDeckLinkAttributeID) -> Result<bool, Error> {
        unsafe {
            let mut v = false;
            match decklink_attributes_get_flag(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error{
                    result: result,
                }),
            }
        }
    }

    pub fn get_supports_internal_keying(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsInternalKeying) }
    pub fn get_supports_external_keying(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsExternalKeying) }
    pub fn get_supports_hd_keying(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsHDKeying) }
    pub fn get_supports_input_format_detection(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsInputFormatDetection) }
    pub fn get_has_reference_input(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasReferenceInput) }
    pub fn get_has_serial_port(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasSerialPort) }
    pub fn get_has_analog_video_output_gain(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasAnalogVideoOutputGain) }
    pub fn get_can_only_adjust_overall_video_output_gain(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkCanOnlyAdjustOverallVideoOutputGain) }
    pub fn get_has_video_input_antialiasing_filter(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasVideoInputAntiAliasingFilter) }
    pub fn get_has_bypass(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasBypass) }
    pub fn get_supports_clock_timing_adjustment(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsClockTimingAdjustment) }
    pub fn get_supports_full_duplex(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsFullDuplex) }
    pub fn get_supports_full_frame_reference_input_timing_offset(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsFullFrameReferenceInputTimingOffset) }
    pub fn get_supports_smpte_level_a_output(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsSMPTELevelAOutput) }
    pub fn get_supports_dual_link_sdi(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsDualLinkSDI) }
    pub fn get_supports_quad_link_sdi(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsQuadLinkSDI) }
    pub fn get_supports_idle_output(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsIdleOutput) }
    pub fn get_has_ltc_timecode_input(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasLTCTimecodeInput) }
    pub fn get_supports_duplex_mode_configuration(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsDuplexModeConfiguration) }
    pub fn get_supports_hdr_metadata(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsHDRMetadata) }
    pub fn get_supports_colorspace_metadata(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsColorspaceMetadata) }

    fn get_int(&self, id: BMDDeckLinkAttributeID) -> Result<i64, Error> {
        unsafe {
            let mut v = 0i64;
            match decklink_attributes_get_int(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error{
                    result: result,
                }),
            }
        }
    }

    pub fn get_maximum_audio_channels(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkMaximumAudioChannels) }
    pub fn get_maximum_analog_audio_input_channels(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkMaximumAnalogAudioInputChannels) }
    pub fn get_maximum_analog_audio_output_channels(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkMaximumAnalogAudioOutputChannels) }
    pub fn get_number_of_subdevices(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkNumberOfSubDevices) }
    pub fn get_subdevice_index(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkSubDeviceIndex) }
    pub fn get_persistent_id(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkPersistentID) }
    pub fn get_device_group_id(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkDeviceGroupID) }
    pub fn get_topological_id(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkTopologicalID) }
    pub fn get_video_output_connections(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkVideoOutputConnections) }
    pub fn get_video_input_connections(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkVideoInputConnections) }
    pub fn get_audio_output_connections(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioOutputConnections) }
    pub fn get_audio_input_connections(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioInputConnections) }
    pub fn get_video_io_support(&self) -> Result<VideoIOSupport, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkVideoIOSupport).map(|v| VideoIOSupport::from_bits_truncate(v as u32)) }
    pub fn get_deck_control_connections(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkDeckControlConnections) }
    pub fn get_device_interface(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkDeviceInterface) }
    pub fn get_audio_input_rca_channel_count(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioInputRCAChannelCount) }
    pub fn get_audio_input_xlr_channel_count(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioInputXLRChannelCount) }
    pub fn get_audio_output_rca_channel_count(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioOutputRCAChannelCount) }
    pub fn get_audio_output_xlr_channel_count(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioOutputXLRChannelCount) }
    pub fn get_paired_device_persistent_id(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkPairedDevicePersistentID) }

    fn get_float(&self, id: BMDDeckLinkAttributeID) -> Result<f64, Error> {
        unsafe {
            let mut v = 0f64;
            match decklink_attributes_get_float(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error{
                    result: result,
                }),
            }
        }
    }

    pub fn get_video_input_gain_minimum(&self) -> Result<f64, Error> { self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkVideoInputGainMinimum) }
    pub fn get_video_input_gain_maximum(&self) -> Result<f64, Error> { self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkVideoInputGainMaximum) }
    pub fn get_video_output_gain_minimum(&self) -> Result<f64, Error> { self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkVideoOutputGainMinimum) }
    pub fn get_video_output_gain_maximum(&self) -> Result<f64, Error> { self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkVideoOutputGainMaximum) }
    pub fn get_microphone_input_gain_minimum(&self) -> Result<f64, Error> { self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkMicrophoneInputGainMinimum) }
    pub fn get_microphone_input_gain_maximum(&self) -> Result<f64, Error> { self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkMicrophoneInputGainMaximum) }

    fn get_string(&self, id: BMDDeckLinkAttributeID) -> Result<String, Error> {
        unsafe {
            let mut v: *mut Buffer = std::ptr::null_mut();
            match decklink_attributes_get_string(self.implementation, id, &mut v) {
                0 => {
                    let ret = Ok(std::ffi::CStr::from_ptr(buffer_data(v) as *const i8).to_str().unwrap_or("").to_string());
                    buffer_release(v);
                    ret
                },
                result => Err(Error{
                    result: result,
                }),
            }
        }
    }

    pub fn get_serial_port_device_name(&self) -> Result<String, Error> { self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkSerialPortDeviceName) }
    pub fn get_vendor_name(&self) -> Result<String, Error> { self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkVendorName) }
    pub fn get_display_name(&self) -> Result<String, Error> { self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkDisplayName) }
    pub fn get_model_name(&self) -> Result<String, Error> { self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkModelName) }
    pub fn get_device_handle(&self) -> Result<String, Error> { self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkDeviceHandle) }
}

impl Drop for Attributes {
    fn drop(&mut self) {
        unsafe {
            decklink_attributes_release(self.implementation);
        }
    }
}

bitflags! {
    pub struct DeviceBusyState: u32 {
        const CAPTURE_BUSY = _BMDDeviceBusyState_bmdDeviceCaptureBusy;
        const PLAYBACK_BUSY = _BMDDeviceBusyState_bmdDevicePlaybackBusy;
        const SERIAL_PORT_BUSY = _BMDDeviceBusyState_bmdDeviceSerialPortBusy;
    }
}

pub struct DisplayModeId(pub u32);

pub struct DisplayMode {
    implementation: *mut IDeckLinkDisplayMode,
}

impl Drop for DisplayMode {
    fn drop(&mut self) {
        unsafe {
            decklink_display_mode_release(self.implementation);
        }
    }
}

impl DisplayMode {
	pub fn get_id(&self) -> DisplayModeId {
        unsafe {
			DisplayModeId(decklink_display_mode_get_display_mode(self.implementation) as u32)
        }
	}

	pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let mut buf: *mut Buffer = std::ptr::null_mut();
            match decklink_display_mode_get_name(self.implementation, &mut buf) {
                0 => {
                    let ret = std::ffi::CStr::from_ptr(buffer_data(buf) as *const i8).to_str().unwrap_or("").to_string();
                    buffer_release(buf);
                    Ok(ret)
                },
                result => Err(Error{
                    result: result,
                }),
            }
        }
	}
}

pub struct DisplayModeIterator {
    implementation: *mut IDeckLinkDisplayModeIterator,
}

impl Drop for DisplayModeIterator {
    fn drop(&mut self) {
        unsafe {
            decklink_display_mode_iterator_release(self.implementation);
        }
    }
}

impl std::iter::Iterator for DisplayModeIterator {
    type Item = DisplayMode;

    fn next(&mut self) -> Option<DisplayMode> {
        unsafe {
            let mut mode: *mut IDeckLinkDisplayMode = std::ptr::null_mut();
            if decklink_display_mode_iterator_next(self.implementation, &mut mode) != 0 || mode.is_null() {
                return None;
            }
            return Some(DisplayMode{
                implementation: mode,
            });
        }
    }
}

pub struct Status {
    implementation: *mut IDeckLinkStatus,
}

impl Status {
    fn get_flag(&self, id: BMDDeckLinkStatusID) -> Result<bool, Error> {
        unsafe {
            let mut v = false;
            match decklink_status_get_flag(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error{
                    result: result,
                }),
            }
        }
    }

    pub fn get_video_input_signal_locked(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkStatusID_bmdDeckLinkStatusVideoInputSignalLocked) }
    pub fn get_reference_signal_locked(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkStatusID_bmdDeckLinkStatusReferenceSignalLocked) }
    pub fn get_received_edid(&self) -> Result<bool, Error> { self.get_flag(_BMDDeckLinkStatusID_bmdDeckLinkStatusReceivedEDID) }

    fn get_int(&self, id: BMDDeckLinkStatusID) -> Result<i64, Error> {
        unsafe {
            let mut v = 0i64;
            match decklink_status_get_int(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error{
                    result: result,
                }),
            }
        }
    }

    pub fn get_detected_video_input_mode(&self) -> Result<DisplayModeId, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusDetectedVideoInputMode).map(|v| DisplayModeId(v as u32)) }
    pub fn get_detected_video_input_flags(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusDetectedVideoInputFlags) }
    pub fn get_current_video_input_mode(&self) -> Result<DisplayModeId, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoInputMode).map(|v| DisplayModeId(v as u32)) }
    pub fn get_current_video_input_pixel_format(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoInputPixelFormat) }
    pub fn get_current_video_input_flags(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoInputFlags) }
    pub fn get_current_video_output_mode(&self) -> Result<DisplayModeId, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoOutputMode).map(|v| DisplayModeId(v as u32)) }
    pub fn get_current_video_output_flags(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoOutputFlags) }
    pub fn get_pci_express_link_width(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusPCIExpressLinkWidth) }
    pub fn get_pci_express_link_speed(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusPCIExpressLinkSpeed) }
    pub fn get_last_video_output_pixel_format(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusLastVideoOutputPixelFormat) }
    pub fn get_reference_signal_mode(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusReferenceSignalMode) }
    pub fn get_reference_signal_flags(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusReferenceSignalFlags) }
    pub fn get_duplex_mode(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusDuplexMode) }
    pub fn get_busy(&self) -> Result<DeviceBusyState, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusBusy).map(|v| DeviceBusyState::from_bits_truncate(v as u32)) }
    pub fn get_interchangeable_panel_type(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusInterchangeablePanelType) }
    pub fn get_device_temperature(&self) -> Result<i64, Error> { self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusDeviceTemperature) }
}

impl Drop for Status {
    fn drop(&mut self) {
        unsafe {
            decklink_status_release(self.implementation);
        }
    }
}

pub struct Iterator {
    implementation: *mut IDeckLinkIterator,
}

impl Drop for Iterator {
    fn drop(&mut self) {
        unsafe {
            decklink_iterator_release(self.implementation);
        }
    }
}

impl std::iter::Iterator for Iterator {
    type Item = Device;

    fn next(&mut self) -> Option<Device> {
        unsafe {
            let mut device: *mut IDeckLink = std::ptr::null_mut();
            if decklink_iterator_next(self.implementation, &mut device) != 0 || device.is_null() {
                return None;
            }
            return Some(Device{
                implementation: device,
            });
        }
    }
}

impl Iterator {
    pub fn new() -> Result<Iterator, SimpleError> {
        unsafe {
            let iterator = create_decklink_iterator_instance();
            if iterator.is_null() {
                return Err(SimpleError::new("unable to create decklink iterator. the latest decklink drivers may need to be installed"));
            }
            return Ok(Iterator{
                implementation: iterator,
            });
        }
    }
}

pub struct Input {
    implementation: *mut IDeckLinkInput,
}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            decklink_input_release(self.implementation);
        }
    }
}

impl Input {
    pub fn get_display_mode_iterator(&mut self) -> Result<DisplayModeIterator, Error> {
        unsafe {
            let mut iterator: *mut IDeckLinkDisplayModeIterator = std::ptr::null_mut();
            match decklink_input_get_display_mode_iterator(self.implementation, &mut iterator) {
				0 => Ok(DisplayModeIterator{
					implementation: iterator,
				}),
				result => Err(Error{
					result: result,
				}),
            }
        }
    }
}

pub struct Output {
    implementation: *mut IDeckLinkOutput,
}

impl Drop for Output {
    fn drop(&mut self) {
        unsafe {
            decklink_output_release(self.implementation);
        }
    }
}

impl Output {
    pub fn get_display_mode_iterator(&mut self) -> Result<DisplayModeIterator, Error> {
        unsafe {
            let mut iterator: *mut IDeckLinkDisplayModeIterator = std::ptr::null_mut();
            match decklink_output_get_display_mode_iterator(self.implementation, &mut iterator) {
				0 => Ok(DisplayModeIterator{
					implementation: iterator,
				}),
				result => Err(Error{
					result: result,
				}),
            }
        }
    }
}
