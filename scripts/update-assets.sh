cd ./assets

wget https://github.com/ahaoboy/mujs-build/archive/refs/tags/v0.0.11.tar.gz
wget https://github.com/ahaoboy/mujs-build/archive/refs/tags/v0.0.11.zip

tar -xzf v0.0.11.tar.gz

tar -cf  test.tar mujs-build-0.0.11
tar -czf test.tar.gz mujs-build-0.0.11
tar -cJf test.tar.xz mujs-build-0.0.11
tar -cjf test.tar.bz2 mujs-build-0.0.11
tar --zstd -cf  test.tar.zst mujs-build-0.0.11
zip -r test.zip mujs-build-0.0.11

rm -rf mujs-build-0.0.11