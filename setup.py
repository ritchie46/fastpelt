from setuptools import setup
import sys
try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding

setup_requires = ["setuptools-rust>=0.10.1", "wheel"]

setup(
    name="fastpelt",
    version="0.1.1",
    author="Ritchie Vink",
    author_email="ritchie46@gmail.com",
    rust_extensions=[RustExtension("fastpelt.fastpeltrust", debug=False, binding=Binding.PyO3)],
    packages=["fastpelt"],
    install_requires=["numpy"],
    include_package_data=True,
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)