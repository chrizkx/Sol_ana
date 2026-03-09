const [tareaPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("tarea-usuario"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);

async function ejecutarPrueba() {
  try {
    console.log("🚀 Iniciando prueba del Gestor de Tareas...");

    // 1. Limpieza inicial (Opcional: borra la tarea si ya existe para probar desde cero)
    const cuentaExistente = await pg.connection.getAccountInfo(tareaPDA);
    if (cuentaExistente) {
      console.log("🧹 Tarea encontrada, eliminándola para reiniciar prueba...");
      await pg.program.methods
        .borrarTarea()
        .accounts({
          tarea: tareaPDA,
          usuario: pg.wallet.publicKey,
        })
        .rpc();
    }

    // 2. CREAR TAREA
    console.log("🛠️ Creando tarea...");
    await pg.program.methods
      .crearTarea("Aprender Solana con Anchor")
      .accounts({
        tarea: tareaPDA,
        usuario: pg.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    let tarea = await pg.program.account.tarea.fetch(tareaPDA);
    console.log("✅ Tarea creada:", tarea.descripcion);
    console.log("📊 Estado completada:", tarea.completada);

    // 3. MARCAR COMO COMPLETADA
    console.log("🔄 Marcando tarea como completada...");
    await pg.program.methods
      .marcarCompletada()
      .accounts({
        tarea: tareaPDA,
        usuario: pg.wallet.publicKey,
      })
      .rpc();

    tarea = await pg.program.account.tarea.fetch(tareaPDA);
    console.log("✅ Nuevo estado completada:", tarea.completada);

    console.log("✨ ¡Prueba finalizada con éxito!");

  } catch (err) {
    console.error("❌ Error durante la ejecución:", err.message);
  }
}

ejecutarPrueba();