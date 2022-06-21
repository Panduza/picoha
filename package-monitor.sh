source package-build.sh
source package-install.sh

SOURCE_DIR=panduza_class_picoha

inotifywait -e close_write,moved_to,create -m -r $SOURCE_DIR |
while read -r directory events filename; do
    echo "++++++++++++++++++ Change on $filename"

    source package-build.sh
    source package-install.sh

done

