import os
import azure.cognitiveservices.speech as speechsdk
import hashlib
import random
from logging import log

def randomly_choose(choices: list):
    return choices[random.randint(0, len(choices) - 1)]

# This example requires environment variables named "SPEECH_KEY" and "SPEECH_REGION"
speech_config = speechsdk.SpeechConfig(
    subscription=os.environ.get('SPEECH_KEY'),
    region=os.environ.get('SPEECH_REGION'))
speech_config.set_speech_synthesis_output_format(speechsdk.SpeechSynthesisOutputFormat.Riff24Khz16BitMonoPcm)  
speech_config.speech_synthesis_voice_name= randomly_choose(['vi-VN-HoaiMyNeural', 'vi-VN-NamMinhNeural'])

def synthesize_sentence(sentence: str, outdir: str) -> str:
    checksum = hashlib.sha256(sentence.encode()).hexdigest()
    filename = f"{outdir}_{checksum}.mp3"
    # if file exists already, don't synthesize it again
    if os.path.exists(filename):
        return filename

    audio_config = speechsdk.audio.AudioOutputConfig(filename=filename)
    speech_synthesizer = speechsdk.SpeechSynthesizer(speech_config=speech_config, audio_config=audio_config)
    speech_synthesis_result = speech_synthesizer.speak_text_async(sentence).get()

    if speech_synthesis_result.reason == speechsdk.ResultReason.SynthesizingAudioCompleted:
        log(0, "Speech synthesized for text [{}]".format(sentence))
        return filename
    elif speech_synthesis_result.reason == speechsdk.ResultReason.Canceled:
        cancellation_details = speech_synthesis_result.cancellation_details
        log(0, "Speech synthesis canceled: {}".format(cancellation_details.reason))
        os.remove(filename)
        if cancellation_details.reason == speechsdk.CancellationReason.Error:
            if cancellation_details.error_details:
                log(0, "Error details: {}".format(cancellation_details.error_details))
                log(0, "Did you set the speech resource key and region values?")