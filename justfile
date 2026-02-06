[working-directory: 'plugins/mock-yt/']
bmock:
    cargo component build --release

[working-directory: 'plugins/mock-provider/']
bmock2:
    cargo component build --release