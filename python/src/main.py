import argparse
import os
import sys
import genanki
import csv

from lib.speak import synthesize_sentence as speak

def parse_csv_to_tuples(filename):
    result = []
    with open(filename, 'r', newline='') as csvfile:
        csvreader = csv.reader(csvfile)
        for row in csvreader:
            if len(row) == 2:  # Ensure there are exactly two columns
                result.append(tuple(row))
    return result

def main():
    if len(sys.argv) != 2:
        print("Usage: python script.py <filename>")
        sys.exit(1)

    filename = sys.argv[1]
    pairs = parse_csv_to_tuples(filename)

    deck = genanki.Deck(
        2059400110,
        'Minimal Pair Training ' + filename)

    model = genanki.Model(
        1607392319,
        'Minimal Pair Training',
        fields=[
            {'name': 'Audio'},
            {'name': 'Word_Left'},
            {'name': 'Word_Right'},
            {'name': 'Correct_Word'},
        ],
        templates=[
            {
            'name': 'Card 1',
            'qfmt': '[sound:{{Audio}}]<hr>{{Word_Left}} VS {{Word_Right}}',
            'afmt': '{{FrontSide}}<hr id="answer">{{Correct_Word}}',
            },
        ]
    )

    out_dir = 'audio'
    audio_files = {}
    media_files = []
    for (a, b) in pairs:
        audio_files[a] = speak(a, out_dir)
        audio_files[b] = speak(b, out_dir)
        deck.add_note(genanki.Note(
            model=model,
            fields=[audio_files[a], a, b, a]))
        deck.add_note(genanki.Note(
            model=model,
            fields=[audio_files[b], a, b, b]))
        media_files.extend([audio_files[a], audio_files[b]])

    package = genanki.Package(deck)
    package.media_files = media_files
    package.write_to_file('minimal_pairs_'+filename+'.apkg')

if __name__ == "__main__":
    main()

