# Build python package
python3 setup.py sdist bdist_wheel

# To upload on pipy
# twine upload dist/*
# twine upload -r testpypi dist/*

