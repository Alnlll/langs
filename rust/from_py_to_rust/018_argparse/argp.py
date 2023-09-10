import argparse
from enum import Enum

class Categories(Enum):
    """
    Enum for the different categories of the dataset.
    """
    Science = 0
    People = 1
    Comedy = 2

VALID_CATEGORIES = [c.name for c in Categories]

def parse_args():
    """
    Parse the command line arguments.
    """
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description='Python argparser demo.')

    parser.add_argument('--file', type=str, required=True,
                        help='Video file to upload.')
    parser.add_argument('--title', type=str,
                        help='Path to the model.')
    parser.add_argument('--category', choices=VALID_CATEGORIES,
                        default='Science',
                        help='Category of the video.')
    parser.add_argument('--verbose', action="store_true", help='Verbose mode.')
    parser.add_argument('-n', '--name', type=str, dest='names', action='append', help='names to greet')

    args = parser.parse_args()
    args.category = Categories.__getitem__(args.category)

    return args

if __name__ == '__main__':
    args = parse_args()
    print(args)