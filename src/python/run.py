from itertools import combinations
import csv, os, locale, sys, shutil

clear = lambda: os.system("cls")
exec(open('floatmerge.py').read()) # opens float merger python to merge all pasted skins and floats & separates the floats to csv file

while input("Press Enter to continue, type Q to Save & Quit\n") == "q":
    clear()
    print("Files saved!")
    quit()

#skinName = input("Please enter the skin that you want to trade up to:\n") # loads minFloat & maxFloat from database (not implemented yet)
skinFloat = float(input("Please enter the skin float that you want:\n"))
minFloat = 0.06 # minimum float from range
maxFloat = 0.80 # maximum float from range
floatRange = (maxFloat - minFloat) # float range of the available skin floats (eg. 0.00-0.99)
bestFloatAverage = (-abs(minFloat) + skinFloat) / floatRange # calculates the float average from the given wanted float and the float range
print("Needed float average is: ", "{:.14f}".format(bestFloatAverage))

def get_terminal_columns():
    return shutil.get_terminal_size().columns # get's the character width of the terminal/console window

input("Press Enter to continue...")
clear()

floats = []
with open("floats.csv") as csvfile: # imports floats from csv files to array
    reader = csv.reader(csvfile, quoting=csv.QUOTE_NONNUMERIC) # change contents to floats
    for row in reader: # each row is a list of floats
        floats.append(row)

comb = combinations(floats, 10)

f = open("combinations.csv", "a")

best = 0 # default latest best float avg
lastDiff = 100 # default last float avg difference
newFloat = 0.000000000000 # possible float from the last/best float average found

floatID = 1 # [n]th float average combination found
combID = 0 # number of combinations done already

for i in comb:
    #print(i, end="", flush=True)


    sumData = 0 # sum of 10 floats in the combination
    for i2 in i:
        sumData += i2[0] # calculates the sum of 10 floats in the combination
        combID += 1 # number of combinations calculated
        print('Progress: ', f'{combID:,}', " combinations \r", end="") # prints the combinations progress into a new line

    newData = sumData/len(i) # calculates the average of [n]th float combination

    # print("The average is: ", sumData/len(i))
    #f.write("{}\n".format(i))

    difference = bestFloatAverage - newData # difference between the wanted float avg and the average of [n]th float combination

    if difference < 0:
        difference = difference*-1 # in case if float avg difference is negative

    if lastDiff > difference:
        best = newData
        lastDiff = difference
        floatID += 1
        newFloat = floatRange * newData + minFloat # calculates the possible skin float from the float average of [n]th float combination
        f.write("ID: ""{:.0f}".format(floatID) + "\\""{:.0f}".format(combID) + "\nNew closest average found: " + repr("{:.14f}".format(newData)) + "\nWanted float average:" + repr("{:.14f}".format(bestFloatAverage)) + "\nFloat average difference: " + repr("{:.14f}".format(difference)) + "\nPossible float from new closest average: " + repr("{:.14f}".format(newFloat)) + "\nCombination:\n")
        f.write("{}\n\n".format(i))
        print(f"\n[{floatID:n}\{combID:n}]\nNew closest average found: ", "{:.14f}".format(newData), "\nWanted float average:", "{:.14f}".format(bestFloatAverage), "\nFloat average difference: ", "{:.14f}".format(difference), "\nPossible float from new closest average: ", "{:.14f}".format(newFloat), "\nCombination:\n", (i), "\n")
        print("*"*(shutil.get_terminal_size().columns), "\n") # draws horizontal "*" separator between found combinations

    if best == bestFloatAverage:
        best = 0
        lastDiff = 100
        print("\033[92m\nExact match found!\nWanted float average:", "{:.14f}".format(bestFloatAverage), "\nFloat average difference: ", "{:.14f}".format(difference), "\nCombination:\n", "{}\n".format(i), "\033[0m")
        f.write("Exact match found!\nWanted float average:" + repr("{:.14f}".format(bestFloatAverage)) + "\nFloat average difference: " + repr("{:.14f}".format(difference)) + "\nCombination:\n")
        f.write("{}\n\n".format(i))

        while input("Press Enter if you want to continue, type Q if you want to Save & Quit\n") == "q":
            print("Combination saved to combinations.txt")
            f.close()
            quit()

input("Press Enter to save float combinations...")
f.close()
clear()