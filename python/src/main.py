import argparse
import os
import sys
import genanki
import csv

from lib.speak import synthesize_sentence as speak

question_format = """
<i>Do you hear</i><br><br>
<div class=container>
<div class=box>{{Word_Left}}
</div>

<div class=or><i> or </i></div>

<div class=box>{{Word_Right}}
</div>
</div>
<br>{{Correct_Audio}}
"""

answer_format = """
<i>Do you hear</i><br><br>
<div class=container>
<div class=box id=box1>{{Word_Left}}
</div>

<div class=or><i> or </i></div>

<div class=box id=box2>{{Word_Right}}
</div>
</div>

<hr id=answer>

You heard: <div class=box>{{Correct_Word}}</div></b><br><br>

{{Audio_Left}}{{Audio_Right}}
<script>
document.getElementById("box1").style.backgroundColor = "#F55"
function flashy()
{
document.getElementById("box1").style.backgroundColor = "#FFF"
document.getElementById("box2").style.backgroundColor = "#F55"
}
function flushy()
{
document.getElementById("box2").style.backgroundColor = "#FFF"
}
setTimeout(flashy,1500);
setTimeout(flushy,3000);
</script>
"""

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
    pairs = parse_csv_to_tuples(filename)[1:]

    pair_name = os.path.basename(filename)
    deck = genanki.Deck(
        2059400110,
        'Minimal Pair Training ' + ' vs '.join(pair_name.split('_')))

    model = genanki.Model(
        1607392319,
        'Minimal Pair Training',
        fields=[
            {'name': 'Audio_Left'},
            {'name': 'Audio_Right'},
            {'name': 'Word_Left'},
            {'name': 'Word_Right'},
            {'name': 'Correct_Word'},
            {'name': 'Correct_Audio'}
        ],
        templates=[
            {
            'name': 'Minimal Pair Card',
            'qfmt': question_format,
            'afmt': answer_format,
            },
        ]
    )

    audio_files = {}
    media_files = []
    mk_ref = lambda audio_file: '[sound:{}]'.format(audio_file)
    for (a, b) in pairs:
        audio_files[a] = speak(a, pair_name)
        audio_files[b] = speak(b, pair_name)
        deck.add_note(genanki.Note(
            model=model,
            fields=[mk_ref(audio_files[a]), mk_ref(audio_files[b]), a, b, a, mk_ref(audio_files[a])]))
        deck.add_note(genanki.Note(
            model=model,
            fields=[mk_ref(audio_files[a]), mk_ref(audio_files[b]), a, b, b, mk_ref(audio_files[b])]))
        media_files.extend([audio_files[a], audio_files[b]])

    package = genanki.Package(deck)
    package.media_files = media_files
    package.write_to_file('minimal_pairs_'+pair_name+'.apkg')

if __name__ == "__main__":
    main()

