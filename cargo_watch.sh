#lembrando que estes comandos ou automações só rodam no bash 
#cmd ou no prompot do windows não vai funcionar

#realizando comando para buildar via watch
cargo_watch.sh
#ele faz o build e já execulta o nosso software.
#este comando: sh cargo_watch.sh
cargo watch -x run

#este comando ele só faz o build
#cargo watch -x build