from openai import OpenAI
from dotenv import load_dotenv
import os

# This script provides an easy option
# to create summaries of the transcribed lectures.
# These summaries could also be embedded or be
# used to enhance the results of the
# semantic search by combining it with keyword search.

instructions = "Du sollst eine Vorlesung zum Thema Berechenbarkeit und Komplexität zusammenfassen. Deine Zusammenfassung ist später wichtig, um auf den ersten Blick zu erkennen, was in einer Vorlesung behandelt wurde, damit bei der Klausurvorbereitung direkt klar ist, welche Vorlesung man sich noch einmal anschauen sollte, wenn man ein bestimmtes Thema nicht verstanden hat. Achte besonders darauf, ob Angaben dazu gemacht werden, ob etwas in der Klausur drankommt. Solche Informationen sollten als [WICHTIG] markiert werden.\n\nFokussiere dich auf Schlagwörter und ignoriere Konversationen und Inhalte, die nicht zum Thema passen. Halte dich kurz aber bleib präzise. Stelle keine Rückfragen. Beende deine Zusammenfassung mit einer Liste an Schlagwörtern, die in einer Keyword Search verwendet werden können."

def main():
    load_dotenv(".env")
    client = OpenAI(api_key=os.getenv("OPENAI_API_KEY"))

    with open("files/input_bk1", "r", encoding="utf-8") as f:
        input_text = f.read()

    result = client.responses.create(
        model="gpt-5-mini",
        instructions=instructions,
        input=input_text,
    )

    print(result.output_text)


if __name__ == "__main__":
    main()
