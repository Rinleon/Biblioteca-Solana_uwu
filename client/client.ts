
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Juegos as Program<any>;
  const wallet = provider.wallet;

  const [jjuegoPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("jjuego"), wallet.publicKey.toBuffer()],
    program.programId
  );

  console.log("PDA:", jjuegoPda.toString());

  //////////////////////////////////////////////////
  // PRUEBAS RAPIDAS
  //////////////////////////////////////////////////

  await crearCuenta();

  await agregarJuego("R.E.P.O", 123, "xbox", "Horror");
  await agregarJuego("Metal slug", 140, "xbox", "ACCION");
  await agregarJuego("Tomb raider", 170, "xbox", "ACCION");
  await cambiarDisponibilidad("R.E.P.O");
  await eliminarJuego("Metal slug");
  await verJuegos();

  async function crearCuenta() {
    await program.methods
      .correrJuego("mi_lista")
      .accounts({
        cuenta: wallet.publicKey,
        jjuego: jjuegoPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Cuenta creada");
  }

  async function agregarJuego(
    nombre: string,
    precio: number,
    plataforma: string,
    genero: string
  ) {
    await program.methods
      .agregarJuego(nombre, precio, plataforma, genero)
      .accounts({
        cuenta: wallet.publicKey,
        jjuego: jjuegoPda,
      })
      .rpc();

    console.log("Juego agregado:", nombre);
  }

  async function cambiarDisponibilidad(nombre: string) {
    await program.methods
      .cambiosJuego(nombre)
      .accounts({
        cuenta: wallet.publicKey,
        jjuego: jjuegoPda,
      })
      .rpc();

    console.log("Disponibilidad cambiada");
  }

  async function eliminarJuego(nombre: string) {
    await program.methods
      .eliminarJuego(nombre)
      .accounts({
        cuenta: wallet.publicKey,
        jjuego: jjuegoPda,
      })
      .rpc();

    console.log("Juego eliminado");
  }

  async function verJuegos() {
    const cuenta = await program.account.tjuego.fetch(jjuegoPda);

    console.log("Lista de juegos:");
    console.log(cuenta.juegos);
  }
}

main();
