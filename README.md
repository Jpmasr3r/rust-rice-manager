# 🦀 Rust Rice Manager (RRM)

*A fast and flexible CLI tool to manage Linux rices using symlinks.*

![License](https://img.shields.io/badge/License-GPL3-red)
![Technology](https://img.shields.io/badge/Technology-Rust-black)
![Status](https://img.shields.io/badge/Status-Active-success)

---

## 📚 Documentation

- [English Documentation](#english-documentation)
- [Documentação em Português](#documentação-em-português)

---

## English Documentation

### Table of Contents

- [🦀 Rust Rice Manager (RRM)](#-rust-rice-manager-rrm)
  - [📚 Documentation](#-documentation)
  - [English Documentation](#english-documentation)
    - [Table of Contents](#table-of-contents)
    - [Description](#description)
    - [Functionality](#functionality)
    - [Installation](#installation)
    - [How to Use](#how-to-use)
      - [File](#file)
      - [Rice](#rice)
      - [Rice File](#rice-file)
    - [Example Usage](#example-usage)
    - [Roadmap](#roadmap)
    - [License](#license)
  - [Documentação em Português](#documentação-em-português)
    - [Descrição](#descrição)
    - [Funcionalidade](#funcionalidade)
    - [Instalação](#instalação)
    - [Como Usar](#como-usar)
      - [File](#file-1)
      - [Rice](#rice-1)
      - [Rice File](#rice-file-1)
    - [Exemplo de Uso](#exemplo-de-uso)
    - [Roteiro de Desenvolvimento](#roteiro-de-desenvolvimento)
    - [Licença](#licença)

---

### Description

RRM (Rust Rice Manager) is a CLI rice manager for Linux focused on ease of use.  
A **“rice”** is a Linux desktop configuration setup — RRM helps you easily manage and switch between them using symbolic links.

---

### Functionality

The main functionality is the creation of file symlinks to the registered file paths, allowing the user to quickly change configuration setups (rices).

---

### Installation

```bash
git clone https://github.com/Jpmasr3r/rust-rice-manager.git
cd rust-rice-manager
cargo build --release
```

---

### How to Use

RRM uses a logic where **Files** store paths to system configuration files (e.g., `hyprland.conf`).  
Then, **Rices** group those files into setups that can be switched dynamically through symlinks.

All aspects can be fully manipulated.

---

#### File

- **Add** → Adds a new file (requires a path and ID).  
- **Remove** → Removes a file (if it doesn’t have a symlink attached, requires the ID).  
- **Update** → Modifies a file’s ID (receives the original and new IDs).  
- **List** → Shows all registered files and their paths.

---

#### Rice

- **Add** → Adds a rice (requires an ID).  
- **Remove** → Removes a rice (requires an ID).  
- **Update** → Updates a rice’s ID (requires original and new IDs).  
- **Change** → Changes the current rice, creating symlinks (requires the rice ID).  
- **File** → Subcommand to link symlinks to a file.  
- **List** → Shows all registered rices.

---

#### Rice File

- **Add** → Adds a new symlink (requires File ID, Rice ID, and the path to the file).  
- **Remove** → Removes a symlink from the rice (requires the File ID).  
- **List** -> List the simlinks of a rice (requires the Rice ID)
- **Update** → Replaces the file saved in `~/.config/rrm/rice/your_rice` (requires Rice ID, File ID, and new file path).

---

### Example Usage

```bash
# Add a file to manage
rrm file add --path ~/.config/hypr/hyprland.conf --id hyprland

# Create a new rice
rrm rice add --id dark-theme

# Link the file to the rice
rrm rice file add --rice dark-theme --file hyprland --path ~/.config/hypr/hyprland.conf

# Apply the rice
rrm rice change --id dark-theme
```

---

### Roadmap

- [x] Basic file and rice management  
- [ ] Backup and restore support  
- [ ] Export/import rices  

---

### License

Distributed under the **GNU GPL v3** license.  
See the `LICENSE` file for more details.

---

## Documentação em Português

### Descrição

RRM (Rust Rice Manager) é um gerenciador de rices em linha de comando para Linux, com foco em facilidade de uso.  
Um **“rice”** é um conjunto de configurações visuais e funcionais de um ambiente Linux. O RRM facilita o gerenciamento e a troca entre essas configurações usando links simbólicos.

---

### Funcionalidade

A principal funcionalidade é a criação de **symlinks (links simbólicos)** para os arquivos registrados, permitindo alternar rapidamente entre diferentes rices.

---

### Instalação

```bash
git clone https://github.com/Jpmasr3r/rust-rice-manager.git
cd rust-rice-manager
cargo build --release
```

---

### Como Usar

O RRM usa uma lógica em que **Files** armazenam caminhos para arquivos de configuração do sistema (por exemplo, `hyprland.conf`).  
Então, **Rices** agrupam esses arquivos em conjuntos que podem ser trocados dinamicamente através de symlinks.

Todos os aspectos podem ser completamente manipulados.

---

#### File

- **Add** → Adiciona um novo arquivo (requer caminho e ID).  
- **Remove** → Remove um arquivo (se não houver symlink anexado, requer o ID).  
- **Update** → Modifica o ID de um arquivo (recebe o ID original e o novo).  
- **List** → Mostra todos os arquivos registrados e seus caminhos.

---

#### Rice

- **Add** → Adiciona um rice (requer um ID).  
- **Remove** → Remove um rice (requer um ID).  
- **Update** → Atualiza o ID de um rice (requer o ID original e o novo).  
- **Change** → Altera o rice atual, criando symlinks (requer o ID do rice).  
- **File** → Subcomando para vincular symlinks a um arquivo.  
- **List** → Mostra todos os rices registrados.

---

#### Rice File

- **Add** → Adiciona um novo symlink (requer ID do Arquivo, ID do Rice e caminho do arquivo).  
- **Remove** → Remove um symlink do rice (requer o ID do Arquivo e o ID do Rice).  
- **List** -> Lista os simlinks de um rice (requer o ID do Rice)
- **Update** → Substitui o arquivo salvo em `~/.config/rrm/rice/your_rice` (requer ID do Rice, ID do Arquivo e novo caminho).

---

### Exemplo de Uso

```bash
# Adicionar um arquivo para gerenciar
rrm file add --path ~/.config/hypr/hyprland.conf --id hyprland

# Criar um novo rice
rrm rice add --id dark-theme

# Vincular o arquivo ao rice
rrm rice file add --rice dark-theme --file hyprland --path ~/.config/hypr/hyprland.conf

# Aplicar o rice
rrm rice change --id dark-theme
```

---

### Roteiro de Desenvolvimento

- [x] Gerenciamento básico de arquivos e rices  
- [ ] Suporte a backup e restauração  
- [ ] Exportar/importar rices  

---

### Licença

Distribuído sob a **licença GNU GPL v3**.  
Consulte o arquivo `LICENSE` para mais detalhes.
