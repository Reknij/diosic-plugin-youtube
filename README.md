# Diosic Plugin Youtube
The plugin is used to resolve the title of youtube music file downloaded by [yt-dlp](https://github.com/yt-dlp/yt-dlp).

Now working resolve:
- remove the video id from title.
- set the cover image by video id.

# Build
```
rustup target add wasm32-wasi

cargo build --target wasm32-wasi --release
```

Copy the target `wasm` file to diosic `<diosic-data-path>/plugins/diosic-plugin-youtube/main.wasm`. (Create directory if not exists.)