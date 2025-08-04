from lektos import MetadataPipeline

class Blog:
    title:str="Unknown"
    author:str="Unknown"


def main():
    html_content = """
<html>
<head>
<title>Example Page</title>
<meta property="og:title" content="OG Title">
</head>
</html>
"""
    pipeline = MetadataPipeline()
    extracted =pipeline.run(html_content)
    print(extracted.title.value if extracted.title else "Unkown")


  


    


if __name__ == "__main__":
    main()
