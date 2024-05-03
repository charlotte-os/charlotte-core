#!/bin/sh

# POSIX ONLY - This script is supposed to run on nearly any distros
# (well at least popular ones)

# CHANGE THIS when there are new requirements
# WARNING: A package might be named differently on different distributions
required_packages_common="nasm xorriso make"

# Here are the requirements with different names on different distros
required_packages_ubuntu="qemu-system"
required_packages_debian="qemu-system-x86"
required_packages_arch="qemu"
required_packages_macos="qemu"

# WARNING: sudo needed

# Wrapper:
#   Print out status of executed command in a human readable way
# Parameters:
#   $1 - Label
#   $2 - Command
# WARNING: String indexing in POSIX is undefined
wrapper() {
    output=$("${@:2}" 2>&1)
    if [ $? -eq 0 ]
    then
        printf "\033[1m[\e[32m Ok \e[0m\033[1m] %s\e[0m\n" "$1"
    else
        printf "\033[1m[\e[31mFail\e[0m\033[1m] %s\e[0m\n       \033[1m\e[31mError\e[0m $output\n" "$1"
        return 1
    fi
}


# Install Rust and Rust nightly toolchain
install_rust_nightly() {
    printf "[    ] Rust nightly toolchain:\n"
    wrapper "    Download Rust installer" $(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >> rust-installer.sh)
    wrapper "    Make Rust installer executable" chmod +x ./rust-installer.sh
    wrapper "    Install Rust nightly" ./rust-installer.sh -y --profile minimal --default-toolchain nightly
    wrapper "    Remove Rust installer" rm rust-installer.sh
}


# Detect OS and distribution
if [ "$(uname)" = "Darwin" ]; then
    for package in $required_packages_common; do
        wrapper "Install \"$package\" package" brew install "$package"
    done

    for package in $required_packages_macos; do
        wrapper "Install \"$package\" package" brew install "$package"
    done
else
    # Check for specific Linux distributions
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        if [ -n "$ID" ]; then
            case "$ID" in
                ubuntu)
                    install_rust_nightly
                    for package in $required_packages_common; do
                        wrapper "Install \"$package\" package" sudo apt-get -y install "$package"
                    done

                    for package in $required_packages_ubuntu; do
                        wrapper "Install \"$package\" package" sudo apt-get -y install "$package"
                    done
                    ;;
                debian)
                    install_rust_nightly
                    for package in $required_packages_common; do
                        wrapper "Install \"$package\" package" sudo apt-get -y install "$package"
                    done

                    for package in $required_packages_debian; do
                        wrapper "Install \"$package\" package" sudo apt-get -y install "$package"
                    done
                    ;;
                arch)
                    install_rust_nightly
                    for package in $required_packages_common; do
                        wrapper "Install \"$package\" package" sudo pacman -S --noconfirm --needed "$package"
                    done

                    for package in $required_packages_arch; do
                        wrapper "Install \"$package\" package" sudo pacman -S --noconfirm --needed "$package"
                    done
                    ;;
                *)
                    printf "\033[1m[\e[31mFail\e[0m\033[1m] Linux distribution is not supported\n"
                    exit 1
                    ;;
            esac
            printf "\033[1m[\e[32m Ok \e[0m\033[1m] Installation done\n"
        else
            printf "\033[1m[\e[31mFail\e[0m\033[1m] Linux distribution could not be determined\n"
        fi
    else
        printf "\033[1m[\e[31mFail\e[0m\033[1m] Linux distribution could not be determined: missing /etc/os-release\n"
    fi
fi