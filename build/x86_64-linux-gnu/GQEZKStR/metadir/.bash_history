mkdir opt-x86_64-linux-musl
mkdir opt-x86_64-linux-musl/registry
ln -s /workspace/srcdir/opt-x86_64-linux-musl/registry /opt/x86_64-linux-musl/registry
mv /tmp .
ln -s /workspace/srcdir/tmp /tmp
apk add fontconfig-dev
