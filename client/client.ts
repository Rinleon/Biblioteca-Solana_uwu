import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

async function main() {

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Juegos as Program<any>;

const wallet = provider.wallet;

const [jjuegoPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("jjuego"),
    wallet.publicKey.toBuffer(),
  ],
  program.programId
);

console.log("PDA:", jjuegoPda.toString());

/* -----------------------------
CREAR CUENTA
------------------------------*/

await program.methods
  .correrJuego("mi_lista")
  .accounts({
    cuenta: wallet.publicKey,
    jjuego: jjuegoPda,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();

console.log("Cuenta creada");


/* -----------------------------
AGREGAR JUEGO
------------------------------*/

await program.methods
  .agregarJuego(
    "Resident Evil 4",
    200,
    "PC",
    "Survival Horror"
  )
  .accounts({
    cuenta: wallet.publicKey,
    jjuego: jjuegoPda,
  })
  .rpc();

console.log("Juego agregado");


/* -----------------------------
VER JUEGOS
------------------------------*/

const cuenta = await program.account.tjuego.fetch(jjuegoPda);

console.log("Juegos guardados:");
console.log(cuenta.juegos);


/* -----------------------------
CAMBIAR DISPONIBILIDAD
------------------------------*/

await program.methods
  .cambiosJuego("Resident Evil 4")
  .accounts({
    cuenta: wallet.publicKey,
    jjuego: jjuegoPda,
  })
  .rpc();

console.log("Disponibilidad cambiada");


/* -----------------------------
ELIMINAR JUEGO
------------------------------*/

await program.methods
  .eliminarJuego("Resident Evil 4")
  .accounts({
    cuenta: wallet.publicKey,
    jjuego: jjuegoPda,
  })
  .rpc();

console.log("Juego eliminado");

}

main();
