# Glossaire

## ABI
Ensemble de conventions définissant comment les programmes interagissent avec le système d'exploitation, le matériel et entre eux (Gestion des registres etc.)
- Exemple : GNU sous Linux

## Target triplet
Triplet spécifiant l'architecture CPU, le vendeur, le système d'exploitation et parfois l'ABI
- Exemple : x86_64-unknow-linux-gnu

## data-layout
Champs présent dans la description d'une target de compilation décrivant les tailles des entiers, flottants, pointeurs...

## Red zone
Zone mémoire spéciale de 128 octets sous le pointeur de pile (RSP) permettant l'optimisation de programme en effectuant des petites opérations locales sans avoir à déplacer le pointeur de pile
Ces opérations ne doivent pas contenir d'interruptions car celles ci peuvent corrompre la mémoire

## VGA Text Buffer
Une zone mémoire spéciale lié au hardware VGA qui contient les informations affichés sur l'écran
Consiste en 25 lignes de 80 caractères

| Bits  | Value            |
|-------|------------------|
| 0-7   | Ascii code       |
| 8-11  | Foreground color |
| 12-14 | Background color |
| 15    | blink            |

| Number | Color      | Number + Bright Bit | Bright Color |
|--------|------------|---------------------|--------------|
| 0x0    | Black      | 0x8                 | Dark Gray    |
| 0x1    | Blue       | 0x9                 | Light Blue   |
| 0x2    | Green      | 0xa                 | Light Green  |
| 0x3    | Cyan       | 0xb                 | Light Cyan   |
| 0x4    | Red        | 0xc                 | Light Red    |
| 0x5    | Magenta    | 0xd                 | Pink         |
| 0x6    | Brown      | 0xe                 | Yellow       |
| 0x7    | Light Gray | 0xf                 | White        |

## Spinlock
Lock qui boucle activement (au lieu de bloquer un thread) jusqu'à ce que le verrou soit disponible. Il est utile dans des situations où on s'attend à ce que le verrou soit libéré très rapidement, mais il est inefficace si la période d'attente est longue, car il gaspille des cycles CPU.

## Universal asynchronous receiver-transmitter (UARTs)

## Serial Ports

## Harness flag (Cargo.toml [test])

## SSE Instructions

## Ou vont les arguments des fonctions en X86_64 Linux ?
En C
Les six premiers int arguments vont dans les registres :
rdi, rsi, rdx, rcx, r8, r9
Les arguments supplémentaires vont sur la pile.
Les résultats sont dans rax et rdx

## Preserved Registers
### C : rbp, rbx rsp, r12, r13, c14, r15
The values of preserved registers must remain unchanged across function calls. So a called function (the “callee”) is only allowed to overwrite these registers if it restores their original values before returning. Therefore, these registers are called “callee-saved”. A common pattern is to save these registers to the stack at the function’s beginning and restore them just before returning.

## Scratch Registers
### C : rax, rcx, rdx, rsi, rdi, r8, r9, r10, r11
A called function is allowed to overwrite scratch registers without restrictions. If the caller wants to preserve the value of a scratch register across a function call, it needs to backup and restore it before the function call (e.g., by pushing it to the stack). So the scratch registers are caller-saved.


## Interrupt Descriptor Table


### Options field
| Bits  | Name                             | Description                                                                                                         |
|-------|----------------------------------|---------------------------------------------------------------------------------------------------------------------|
| 0-2   | Interrupt Stack Table Index      | 0: Don’t switch stacks, 1-7: Switch to the n-th <br>stack in the Interrupt Stack Table when this handler is called. |
| 3-7   | Reserved                         |                                                                                                                     |
| 8     | 0: Interrupt Gate, 1: Trap Gate  | If this bit is 0, interrupts are disabled when this handler is called.                                          |
| 9-11  | must be one                      |                                                                                                                     |
| 12    | must be zero                     |                                                                                                                     |
| 13‑14 | Descriptor Privilege Level (DPL) | The minimal privilege level required <br>for calling this handler.                                                  |
| 15    | Present                          |                                                                                                                     |

