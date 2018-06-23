import Control.DeepSeq
import Control.Exception
import System.TimeIt

readLines = fmap lines . readFile

main = do
    words <- readLines "words.txt"
    shuffled_words <- readLines "shuffled_words.txt"
    evaluate (rnf words)  -- Do not measure readLines...
    evaluate (rnf shuffled_words)
    let present = all (`elem` words) shuffled_words
    timeIt $ print present
