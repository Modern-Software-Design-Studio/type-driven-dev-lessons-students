for file in *.md ;
    do pandoc $file -o "${file%%.*}".pdf  ; 
done