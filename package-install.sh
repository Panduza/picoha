# PACKAGE=panduza_platform
# echo "!! UNINSTALL $PACKAGE !!"

tar_filepath=`readlink -f ./dist/panduza*.tar.gz`
echo "Update from : $tar_filepath"
python3 setup.py sdist bdist_wheel
# sudo pip uninstall -y $PACKAGE
sudo pip install --upgrade $tar_filepath
