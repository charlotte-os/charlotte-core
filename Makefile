#x86_64

limine:
	@if [ ! -d "limine" ]; then \
		git clone https://github.com/limine-bootloader/limine.git --branch=v5.x-branch-binary --depth=1;\
	fi
	make -C limine

ovmf-x86_64:
	mkdir -p ovmf-x86_64
	cd ovmf-x86_64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

build-x86_64-debug: limine
	cd charlotte_core && cargo build --target x86_64-unknown-none
	rm -rf iso_root
	mkdir -p iso_root
	cp -v charlotte_core/target/x86_64-unknown-none/debug/charlotte_core \
		limine.cfg limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o charlotte_core-x86_64-debug.iso
	rm -rf iso_root

run-x86_64-debug: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 2G -bios ovmf-x86_64/OVMF.fd -cdrom charlotte_core-x86_64-debug.iso -boot d -serial stdio

run-x86_64-extdb: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -s -S -M q35 -m 2G -bios ovmf-x86_64/OVMF.fd -cdrom charlotte_core-x86_64-debug.iso -boot d

run-x86_64-log: ovmf-x86_64 build-x86_64-debug
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 12G -bios ovmf-x86_64/OVMF.fd -cdrom charlotte_core-x86_64-debug.iso -boot d -serial file:log_x86_64.txt

build-x86_64-release: limine
	cd charlotte_core && cargo build --target x86_64-unknown-none --release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v charlotte_core/target/x86_64-unknown-none/release/charlotte_core \
		limine.cfg limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o charlotte_core-x86_64-release.iso
	rm -rf iso_root

run-x86_64-release: ovmf-x86_64 build-x86_64-release
	qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 12G -bios ovmf-x86_64/OVMF.fd -cdrom charlotte_core-x86_64-release.iso -boot d

check-x86_64:
	cd charlotte_core && cargo check --target x86_64-unknown-none

# aarch64

ovmf-aarch64:
	mkdir -p ovmf-aarch64
	cd ovmf-aarch64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEAARCH64_QEMU_EFI.fd
build-aarch64-debug: limine
	cd charlotte_core && cargo build --target aarch64-unknown-none
charlotte_core-aarch64-debug.iso: build-aarch64-debug
	rm -rf iso_root
	mkdir -p iso_root
	cp -v charlotte_core/target/aarch64-unknown-none/debug/charlotte_core \
		limine.cfg limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTAA64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o charlotte_core-aarch64-debug.iso
	rm -rf iso_root
run-aarch64-debug: ovmf-aarch64 charlotte_core-aarch64-debug.iso
	qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom charlotte_core-aarch64-debug.iso -boot d
run-aarch64-log: ovmf-aarch64 charlotte_core-aarch64-debug.iso
	qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom charlotte_core-aarch64-debug.iso -boot d \
		-serial file:log_aarch64.txt


build-aarch64-release: limine
	cd charlotte_core && cargo build --target aarch64-unknown-none --release
charlotte_core-aarch64-release.iso: build-aarch64-release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v charlotte_core/target/aarch64-unknown-none/release/charlotte_core \
		limine.cfg limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTAA64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o charlotte_core-aarch64-release.iso
	rm -rf iso_root
run-aarch64-release: ovmf-aarch64 charlotte_core-aarch64-release.iso
	qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -m 2G -bios ovmf-aarch64/OVMF.fd -cdrom charlotte_core-aarch64-release.iso -boot d

# riscv64

ovmf-riscv64:
	mkdir -p ovmf-riscv64
	cd ovmf-riscv64 && curl -o OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASERISCV64_VIRT_CODE.fd && dd if=/dev/zero of=OVMF.fd bs=1 count=0 seek=33554432
build-riscv64-debug:
	cd charlotte_core && cargo build --target riscv64gc-unknown-none-elf
charlotte_core-riscv64-debug.iso: build-riscv64-debug
	rm -rf iso_root
	mkdir -p iso_root
	cp -v charlotte_core/target/riscv64gc-unknown-none-elf/debug/charlotte_core \
		limine.cfg limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTRISCV64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o charlotte_core-riscv64-debug.iso
	rm -rf iso_root
run-riscv64-debug: ovmf-riscv64 charlotte_core-riscv64-debug.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=charlotte_core-riscv64-debug.iso
run-riscv64-debug: ovmf-riscv64 charlotte_core-riscv64-debug.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=charlotte_core-riscv64-debug.iso \
		-serial file:log_riscv64.txt

build-riscv64-release:
	cd charlotte_core && cargo build --target riscv64gc-unknown-none-elf
charlotte_core-riscv64-release.iso: build-riscv64-release
	rm -rf iso_root
	mkdir -p iso_root
	cp -v charlotte_core/target/riscv64gc-unknown-none-elf/release/charlotte_core \
		limine.cfg limine/limine-uefi-cd.bin iso_root/
	mkdir -p iso_root/EFI/BOOT
	cp -v limine/BOOTRISCV64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o charlotte_core-riscv64-release.iso
	rm -rf iso_root
run-riscv64-release: ovmf-riscv64 charlotte_core-riscv64-release.iso
	qemu-system-riscv64 -M virt -cpu rv64 \
		-device ramfb -device qemu-xhci -device usb-kbd -m 2G -drive if=pflash,unit=0,format=raw,file=ovmf-riscv64/OVMF.fd \
		-device virtio-scsi-pci,id=scsi -device scsi-cd,drive=cd0 -drive id=cd0,format=raw,file=charlotte_core-riscv64-release.iso

# clean commands

clean:
	cd charlotte_core && cargo clean
	rm -rf ovmf-aarch64
	rm -rf ovmf-riscv64
	rm -rf ovmf-x86_64
	rm -f charlotte_core-aarch64-debug.iso
	rm -f charlotte_core-riscv64-debug.iso
	rm -f charlotte_core-x86_64-debug.iso
	rm -f charlotte_core-aarch64-release.iso
	rm -f charlotte_core-riscv64-release.iso
	rm -f charlotte_core-x86_64-release.iso
	rm -f log_aarch64.txt
	rm -f log_riscv64.txt
	rm -f log_x86_64.txt

distclean: clean
	rm -rf limine
