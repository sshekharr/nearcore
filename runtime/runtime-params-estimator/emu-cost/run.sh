docker run \
     --mount type=bind,source=$HOST_DIR,target=/host \
     -i -t rust-emu \
     /bin/bash