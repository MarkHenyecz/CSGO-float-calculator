import shutil, time, glob, os, csv

clear = lambda: os.system('cls')
filenames = glob.glob('*.txt')
outfilename = ('skinlist.txt')
combfile = ('combinations.csv')
output = ('floats.csv')

clear()
print("Combinating the next files:")
with open(outfilename, 'wb') as outfile:
    for filename in glob.glob('*.txt'): # checks all txt files
        if (filename == outfilename and output and combfile): continue # don't want to copy the output into the output or other files
        with open(filename, 'rb') as readfile:
            for line in readfile:
                if not line.strip(): 
                    continue
                if line.startswith((b'Get Float Get sticker wear', b'CS.Money 3D', b'Screenshot', b'Buy Now', b'Paint Seed:', b'Counter-Strike')):
                    continue
                if ('%'.encode('utf-8')) in line:
                    continue
                if b'\xe2\x82\xac' in line:
                    outfile.write(line + b'\n') 
                else:
                    outfile.write(line)
            outfile.write(b'\n')
        print (filename)
        outfile.write(b'\n')
print("\033[92mFile combination finished!\033[0m")
delete = ("Float: ")
with open(outfilename, encoding="utf8") as infile, open(output, 'w') as outfile:
    for line in infile:
        if not line.strip(): 
            continue  # skip the empty line
        if line.startswith(delete): # Checks if the line starts with "Float: "
            line = line.replace(delete, "")
            outfile.write(line)  # non-empty line. Write it to output

#input("Press Enter to continue...")
#clear()