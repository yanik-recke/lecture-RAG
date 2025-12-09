import grpc
from concurrent import futures
import os
from dotenv import load_dotenv
from openai import OpenAI

import target.summary_service_pb2 as summary_pb2
import target.summary_service_pb2_grpc as summary_grpc

try:
    load_dotenv(".env")
except:
    print("No .env file found.")


# Instructions for summarizing lectures
INSTRUCTIONS = """Du sollst eine Vorlesung zum Thema Berechenbarkeit und Komplexität zusammenfassen. Deine Zusammenfassung ist später wichtig, um auf den ersten Blick zu erkennen, was in einer Vorlesung behandelt wurde, damit bei der Klausurvorbereitung direkt klar ist, welche Vorlesung man sich noch einmal anschauen sollte, wenn man ein bestimmtes Thema nicht verstanden hat. Achte besonders darauf, ob Angaben dazu gemacht werden, ob etwas in der Klausur drankommt. Solche Informationen sollten als [WICHTIG] markiert werden.

Fokussiere dich auf Schlagwörter und ignoriere Konversationen und Inhalte, die nicht zum Thema passen. Halte dich kurz aber bleib präzise. Stelle keine Rückfragen. Beende deine Zusammenfassung mit einer Liste an Schlagwörtern, die in einer Keyword Search verwendet werden können."""


class SummaryServiceServicer(summary_grpc.SummaryServiceServicer):

    def __init__(self):
        """Initialize the OpenAI client."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            raise Exception("Missing OPENAI_API_KEY environment variable")

        self.client = OpenAI(api_key=api_key)
        print("OpenAI client initialized successfully")

    def Summarize(self, request, context):
        """
        Summarize text using OpenAI API.

        Args:
            request: SummarizeReq containing text to_summarize
            context: gRPC context

        Returns:
            SummarizeRes containing either summary or error_msg
        """
        try:
            text_to_summarize = request.to_summarize

            if not text_to_summarize:
                return summary_pb2.SummarizeRes(
                    error_msg="Empty text provided for summarization"
                )

            print(f"Summarizing text of length: {len(text_to_summarize)} characters")

            # Call OpenAI API to create summary
            result = self.client.responses.create(
                model="gpt-5-mini",
                instructions=INSTRUCTIONS,
                input=text_to_summarize,
            )

            summary_text = result.output_text

            print(f"Summary generated successfully: {len(summary_text)} characters")

            return summary_pb2.SummarizeRes(summary=summary_text)

        except Exception as e:
            error_message = f"Error during summarization: {str(e)}"
            print(error_message)
            context.set_code(grpc.StatusCode.INTERNAL)
            context.set_details(error_message)
            return summary_pb2.SummarizeRes(error_msg=error_message)


def serve():
    """Start the gRPC server."""
    port = os.environ.get("SUMMARY_PORT", "50053")

    # Set max message size to 100 MB to handle large texts
    max_message_length = 100 * 1024 * 1024  # 100 MB in bytes
    options = [
        ('grpc.max_send_message_length', max_message_length),
        ('grpc.max_receive_message_length', max_message_length),
    ]

    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10), options=options)

    summary_grpc.add_SummaryServiceServicer_to_server(
        SummaryServiceServicer(), server
    )

    server.add_insecure_port(f'[::]:{port}')
    server.start()

    print(f"Summary gRPC server started on port {port}")

    try:
        server.wait_for_termination()
    except KeyboardInterrupt:
        print("\nShutting down server...")
        server.stop(0)


if __name__ == '__main__':
    serve()
