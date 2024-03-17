#!/usr/bin/bash -x

efi_name=""

case $1 in
  x86_64)
    efi_name="BOOTX64.EFI"
    ;;

  aarch64)
    efi_name="BOOTAA64.EFI"
    ;;

  *)
    echo "Invalid arch '$1'"
    exit 1
    ;;
esac

echo $2

IMAGE_NAME="charlotte_core-$1-test.iso"

cp -v $2 limine.cfg limine/limine-uefi-cd.bin iso_root/

rm -rf iso_root
mkdir -p iso_root
cp -v $2 iso_root/charlotte_core
cp -v ../limine.cfg ../limine/limine-uefi-cd.bin iso_root/
mkdir -p iso_root/EFI/BOOT
cp -v ../limine/$efi_name iso_root/EFI/BOOT/

xorriso -as mkisofs \
	-no-emul-boot -boot-load-size 4 -boot-info-table \
	--efi-boot limine-uefi-cd.bin \
	-efi-boot-part --efi-boot-image --protective-msdos-label \
	iso_root -o $IMAGE_NAME

rm -rf iso_root

case $1 in
  x86_64)
    qemu-system-x86_64 -M q35 \
                       -m 2G \
                       -bios ../ovmf-x86_64/OVMF.fd \
                       -cdrom $IMAGE_NAME \
                       -boot d \
                       -serial stdio
    ;;

  aarch64)
     qemu-system-aarch64 -M virt \
                         -cpu cortex-a72 \
                         -device ramfb \
                         -device qemu-xhci \
                         -device usb-kbd \
                         -m 2G -bios ../ovmf-aarch64/OVMF.fd \
                         -cdrom $IMAGE_NAME -boot d
    ;;

  *)
    ;;
esac


