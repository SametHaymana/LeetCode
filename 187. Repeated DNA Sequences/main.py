import hashlib
from typing import List

    

class Solution:
    
    def __hash(self, s:str) -> str:
        return hashlib.md5(s.encode()).hexdigest()
    
    def findRepeatedDnaSequences(self, s: str) -> List[str]:
        if len(s) < 10:
            return []
        
        result = set()
        seen = set()
        for i in range(len(s) - 9):
            sub = s[i:i+10]
            if self.__hash(sub) in seen:
                result.add(sub)
            else:
                seen.add(self.__hash(sub))
                
        return result
    
sol = Solution()
print(sol.findRepeatedDnaSequences("AAAAAAAAAAAAA"))