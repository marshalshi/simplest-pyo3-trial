from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="rust2py",
    version="1.0",
    rust_extensions=[RustExtension("rust2py.rust2py", binding=Binding.PyO3)],
    packages=["rust2py"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
