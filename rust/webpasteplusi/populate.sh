for i in http https ftp ssh smtp;do for j in {0..65535}; do echo $j\$$i >> populate.txt; done done
