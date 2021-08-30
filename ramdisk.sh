sudo mount -t tmpfs none ./target&&cat /proc/mounts | grep "$(pwd)" | sudo tee -a /etc/fstab


