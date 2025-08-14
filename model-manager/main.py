import csv, subprocess

def clone_with_git_link(link:str):
    print(f"Getting {link}")


def main():
    print("Hello from model-manager!")
    with open('models.csv' , 'r') as models:
        keys = models.readline().strip().split(',')
        print(keys)
        lines= models.readlines()
        for line in lines:
            for (k , v) in zip(line.split(',') , keys):
                if k == 'Link': 
                    if v.contains("huggingface.co"): clone_with_git_link(v.strip())
                    else: print("not a model link")

if __name__ == "__main__":
    main()
