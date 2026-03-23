🎬 C.VIP TIME - Solana Cinema Experience
C.VIP TIME es una plataforma descentralizada diseñada para revolucionar la logística de servicios VIP en salas de cine. Utilizando la infraestructura de Solana, eliminamos los errores de comunicación entre el cliente y la cocina, garantizando que cada pedido llegue caliente y al asiento correcto, validado por Smart Contracts.

🚀 Características Principales (Funcionalidad)
Gestión On-Chain: Registro inmutable de pedidos, asientos y estados de cocina.

Validación de Presencia: El sistema bloquea el proceso de cocina si el cliente no ha confirmado su llegada a la sala (Seguridad lógica).

Monitor en Tiempo Real: Interfaz para el Staff que organiza pedidos por prioridad y estado de llegada.

Portal VIP Cliente: Feedback visual instantáneo (efecto glow) cuando el pedido está listo para entrega.

🛠️ Stack Tecnológico (Estructura)
Smart Contract: Escrito en Rust utilizando el framework Anchor.

Blockchain: Desplegado en Solana Devnet.

Frontend: Single Page Applications (SPA) desarrolladas con HTML5, CSS3 y JavaScript nativo.

Integración Web3: Conexión directa vía @solana/web3.js (sin servidores intermedios).

🏗️ Arquitectura del Código
El proyecto se divide en tres componentes críticos:

El Programa (Rust): Ubicado en /programs/cvip_time. Maneja la lógica de estados (Espera, Cocina, Listo) y las restricciones de seguridad.

Monitor Staff (Operaciones): Sistema de consulta masiva de cuentas para la gestión de cocina.

Portal VIP (UX): Interfaz reactiva para el usuario final con actualización automática cada 5 segundos.

Program ID: 38dxXpL8wvgzr4Ai12nvpRDdEw3RzVvBgh5CgBqjYyx6

🎥 Tutorial


🔧 Cómo probar el proyecto
Navega a la carpeta del Portal VIP o Monitor Staff.

Abre los archivos .html en cualquier navegador moderno.

Ingresa una Address de Ticket válida de la Devnet (Ejemplo: [FHLkCDudaGs8RVSke3uXZ476egE3YdPFqhsPHxkiL42b]) para ver la sincronización en tiempo real con la blockchain.
