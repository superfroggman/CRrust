let input_path_cstr = std::ffi::CString::new("/home/anton/Videos/video.mp4").unwrap();

// Open an e.g. MP4 file
avformat_open_input(
    &mut ifmt_ctx,
    input_path_cstr.as_ptr(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
);
avformat_find_stream_info(ifmt_ctx, std::ptr::null_mut());

// Print info about the loaded file
av_dump_format(
    ifmt_ctx,
    0,
    input_path_cstr.as_ptr(),
    0,
);
