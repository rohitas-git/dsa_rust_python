class MyHash:
    def __init__(self, c):
        self.cap = c
        self.table = [-1] * c
        self.size = 0

    def hash(self, x):
        return x % self.cap

    def search(self, x):
        h = self.hash(x)
        t = self.table
        i = 0
        res = h + i
        while t[res] != -1:
            if t[res] == x:
                return True
    
            i+=1
            res = doubleHashing(x,i,self.cap)
            
            # Linear Probing
            # res = (res + 1) % self.cap
    
            # Double Hashing
            # res = (res + (x % 6)) % self.cap 
            
            # Quadratic
            # res = (h + i*i)% self.cap
            if res == h:
                return False
        return False

    def insert(self, x):
        if self.size == self.cap:
            return False

        if self.search(x) == True:
            return False
        i = self.hash(x)
        t = self.table
        while t[i] not in (-1, -2):
            # i = (i + 1) % self.cap
            i = doubleHashing(x,i,self.cap)

        t[i] = x
        self.size = self.size + 1
        return True

    def remove(self, x):
        h = self.hash(x)
        t = self.table
        i = h
        while t[i] != -1:
            if t[i] == x:
                t[i] = -2
                return True
            # i = (i + 1) % self.cap
            i = doubleHashing(x,i,self.cap)
            if i == h:
                return False
        return False


def linearProbing(i, m):
    i = (i + 1) % m
    

def doubleHashing(key, i ,m):
    i = (i + (key % 6)) % m

    
def quadraticProbing(key, i,m):
    i = (key%m + i*i) % m


h = MyHash(7)
h.insert(70)
h.insert(71)
h.insert(9)
h.insert(56)
h.insert(72)
print(h.search(56))
h.remove(56)
print(h.search(56))
h.remove(56)