use hydrust_sdk::register_plugin;

register_plugin!(MockPlugin);

struct MockPlugin;

impl Guest for MockPlugin {
    fn can_handle(url: String) -> bool {
        url.contains("example.com")
    }

    fn get_stream(_url: String) -> Result<StreamInfo, ErrorCode> {
        Ok(StreamInfo {
            title: "Hydrust Mock Stream".to_string(),
            url: "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8".to_string(),
        })
    }
}