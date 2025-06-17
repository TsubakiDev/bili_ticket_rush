CARGO ?= cargo
FLAGS = RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none"
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release

TARGET ?= 

X86_WINDOWS  := x86_64-pc-windows-gnu
X86_LINUX	 :- x86_64-unknown-linux-gnu
ARM_MACOS    := aarch64-apple-darwin

.PHONY: build
build:
	$(FLAGS) $(CARGO) build -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" --release $(if $(TARGET),--target $(TARGET),)

.PHONY: windows linux macos
windows:
	@$(MAKE) build TARGET=$(X86_WINDOWS)
	
linux:
	@$(MAKE) build TARGET=$(X86_LINUX)
	
macos:
	@$(MAKE) build TARGET=$(X86_MACOS)

.PHONY: all
all: windows linux macos

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: setup-linux
setup-linux:
	sudo apt-get update
	sudo apt-get install -y \
		g++-mingw-w64-x86-64 \
		libc6-dev-i386

.PHONY: rustup-add-targets
rustup-add-targets:
	rustup target add $(X86_WINDOWS)
	rustup target add $(X86_LINUX)
	rustup target add $(X86_MACOS)
	rustup target add $(ARM_LINUX)
	rustup target add $(ARM_MACOS)

.PHONY: help
help:
	@echo "可用命令:"
	@echo "  build           - 构建当前平台 (默认)"
	@echo "  windows         - 构建 Windows 目标"
	@echo "  linux           - 构建 Linux 目标"
	@echo "  macos           - 构建 macOS 目标"
	@echo "  all             - 构建所有平台"
	@echo "  clean           - 清理构建文件"
	@echo "  setup-linux     - 安装交叉编译工具 (Linux 主机)"
	@echo "  rustup-add-targets - 添加所有目标平台支持"