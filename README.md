# Lex Compiladores

### Overview
This project is a **lexical analyzer (lexer)** for a domain-specific language (DSL) designed for modeling reactive systems and finite state machines. The lexer tokenizes source files written in a custom language and generates a token stream for compiler stages.

### Project Structure
- **lex.py** - Main lexical analyzer written in Python. Reads source files and generates tokens.
- **\*.rs** - Example programs written in the DSL:
  - `ae.rs` - Highway traffic control system ("Auto Estrada")
  - `lamp.rs` - Light control system
  - `login.rs` - Login/authentication system
  - `podos.rs` - Example program
  - `rst.rs` - Example program
- **trab1.py** - Secondary Python script (possibly for testing or utilities)
- **output.txt** - Generated token output

### Language Features
The DSL supports:
- **Module definitions** - Nested reactive components
- **Input/Output signals** - Communication between modules
- **Temporal signals** (t_signal) and Persistent signals** (p_signal)
- **Variables and state management** - Local variable declarations
- **Event handling** - Rules triggered by specific events
- **Control structures** - Case statements and conditional logic
- **Operators** - Comparison (==>, --->), arithmetic, and logical operations

### How to Use

#### Running the Lexer
```bash
python lex.py <input_file.rs>
```

**Example:**
```bash
python lex.py ae.rs
```

This will generate tokens and save them to `output.txt`.

#### Token Output
The lexer generates the following token types:
- **Reserved words** - Language keywords (module, input, output, var, etc.)
- **Operators** - ==>, --->, =, >, <, >=, <=, +, -, &, /
- **Separators** - [], (), {}, ., :, #, ,, '
- **Identifiers** - Variable and function names
- **Numbers** - Numeric literals
- **Comments** - /* ... */ style comments

### Example Program
The highway control system (`ae.rs`) demonstrates:
- Multi-module architecture
- Signal passing between components
- Conditional state transitions
- Variable management and initialization

### Requirements
- Python 3.x

### License
This project is part of a compiler course/assignment.

---

## 🇧🇷

### Visão Geral
Este projeto é um **analisador léxico (lexer)** para uma linguagem de domínio específico (DSL) projetada para modelar sistemas reativos e máquinas de estados finitos. O lexer tokeniza arquivos fonte escritos em uma linguagem customizada e gera um fluxo de tokens para os estágios do compilador.

### Estrutura do Projeto
- **lex.py** - Analisador léxico principal escrito em Python. Lê arquivos fonte e gera tokens.
- **\*.rs** - Programas de exemplo escritos em DSL:
  - `ae.rs` - Sistema de controle de trânsito em auto-estrada
  - `lamp.rs` - Sistema de controle de iluminação
  - `login.rs` - Sistema de login/autenticação
  - `podos.rs` - Programa de exemplo
  - `rst.rs` - Programa de exemplo
- **trab1.py** - Script Python secundário (possivelmente para testes ou utilitários)
- **output.txt** - Saída de tokens gerada

### Características da Linguagem
A DSL suporta:
- **Definições de módulos** - Componentes reativos aninhados
- **Sinais de entrada/saída** - Comunicação entre módulos
- **Sinais temporais** (t_signal) e **sinais persistentes** (p_signal)
- **Variáveis e gerenciamento de estado** - Declarações de variáveis locais
- **Tratamento de eventos** - Regras disparadas por eventos específicos
- **Estruturas de controle** - Instruções case e lógica condicional
- **Operadores** - Comparação (==>, --->), aritmética e lógica

### Como Usar

#### Executando o Lexer
```bash
python lex.py <arquivo_entrada.rs>
```

**Exemplo:**
```bash
python lex.py ae.rs
```

Isto gerará tokens e os salvará em `output.txt`.

#### Saída de Tokens
O lexer gera os seguintes tipos de tokens:
- **Palavras reservadas** - Palavras-chave da linguagem (module, input, output, var, etc.)
- **Operadores** - ==>, --->, =, >, <, >=, <=, +, -, &, /
- **Separadores** - [], (), {}, ., :, #, ,, '
- **Identificadores** - Nomes de variáveis e funções
- **Números** - Literais numéricos
- **Comentários** - Comentários no estilo /* ... */

### Programa de Exemplo
O sistema de controle de auto-estrada (`ae.rs`) demonstra:
- Arquitetura com múltiplos módulos
- Passagem de sinais entre componentes
- Transições de estado condicionais
- Gerenciamento de variáveis e inicialização

### Requisitos
- Python 3.x

### Licença
Este projeto faz parte de um curso/trabalho de compiladores.
