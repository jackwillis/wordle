fn main() {
    // Statically link the Visual C++ runtime when using the MSVC toolchain.
    // See <https://github.com/ChrisDenton/static_vcruntime>.
    static_vcruntime::metabuild();
}
