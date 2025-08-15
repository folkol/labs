import Data.Maybe
import Data.List
import System.TimeIt

readLines = fmap lines . readFile

main = do
    words <- readLines "words.txt"
    shuffledWords <- readLines "shuffled_words.txt"
    -- let indexes = map (`elemIndex` words) shuffledWords
    mapM_ print $ catMaybes $ map (`elemIndex` words) shuffledWords

