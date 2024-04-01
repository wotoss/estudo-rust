# Linu ou subsistem wsl
# sudo apt install lldb


# Comando para compilar o código Rust com informações de depuração
cargo build

# Verifica se a compilação foi bem-sucedida
if [ $? -eq 0 ]; then
    # Inicia o LLDB para depurar o executável gerado
    lldb target/debug/console
else
    echo "Erro ao compilar o código Rust"
fi