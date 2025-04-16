# Montar proyecto

# Permisos para comunicarte con el ESP32
    sudo usermod -aG dialout $USER
 Recuerda cerrar sesión o reiniciar para que los cambios tengan efecto.

## Instalar rust y dependencias de linux

### Rust
[Web oficial de RUST](https://rustup.rs/)

### Requerido para una dependencia de cargo
    
    sudo dnf install systemd-devel

## Instalara XTensa
[Manual completo en GitHub](https://github.com/esp-rs/rust-build?tab=readme-ov-file#espup-installation)

    curl -L https://github.com/esp-rs/espup/releases/latest/download/espup-x86_64-unknown-linux-gnu -o espup
    chmod a+x espup
    ./espup install
    # Source the following file in every terminal before building a project
    . $HOME/export-esp.sh

## Instalar paquetes cargo

    cargo install espflash

    cargo install cargo-generate

    cargo install ldproxt

## Crear plantilla de proyecto

    # STD Project
    cargo generate esp-rs/esp-idf-template cargo
    # NO-STD (Bare-metal) Project
    cargo generate esp-rs/esp-template



## Build

Conectalo en el de la **DERECHA!!!** y ya te preguntara el puerto el siguiente comando

    cargo run