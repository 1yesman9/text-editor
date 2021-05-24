/*
    INTERFACE:
        pub fn left_string -> &[u8] ( returns left string buffer[0..gap-left])
        pub fn right_string -> &[u8] ( returns right_string: buffer[gap_right..])
        pub fn new() -> creates new gap buffer
        pub fn insert() -> inserts at cursor pos ( how is cursor represented? )
*/

/*
// C++ program of implementation of gap buffer
  
#include <bits/stdc++.h> 
using namespace std; 
  
YESMAN: MAKE GAP STRUCTURE, AND WRITE THESE AS METHODS
char buffer[50]; 
int gap_size = 10; 
int gap_left = 0; 
int gap_right = gap_size - gap_left-1; 
int size = 10; 
  
// Function that is used to grow the gap 
// at index position and return the array 
  
  
void grow(int k, int position) 
{ 
  
    char a[size]; 
  
    // Copy characters of buffer to a[] 
    // after position 
    for (int i = position; i < size; i++) { 
        a[i - position] = buffer[i]; 
          
    } 
      
    // Insert a gap of k from index position 
    // gap is being represented by '-' 
    for (int i = 0; i < k; i++) { 
        buffer[i + position] = '_'; 
    } 
      
    // Reinsert the remaining array 
    for (int i = 0; i < position + k; i++) { 
        buffer[position + k + i] = a[i]; 
    } 
  
    size += k;
    gap_right+=k;
} 
  
// Function that is used to move the gap 
// left in the array 
void left(int position) 
{ 
    // Move the gap left character by character 
    // and the buffers 
    while (position < gap_left) { 
        gap_left--; 
        gap_right--; 
        buffer[gap_right+1] = buffer[gap_left];
        buffer[gap_left]='_';
    } 
} 
  
// Function that is used to move the gap 
// right in the array 
void right(int position) 
{ 
    // Move the gap right character by character 
    // and the buffers 
    while (position > gap_left) { 
        gap_left++; 
        gap_right++; 
        buffer[gap_left-1] = buffer[gap_right]; 
        buffer[gap_right]='_';
    } 
} 
  
// Function to control the movement of gap 
// by checking its position to the point of 
// insertion 
void move_cursor(int position) 
{ 
    if (position < gap_left) { 
        left(position); 
    } 
    else { 
        right(position); 
    } 
} 
  
// Function to insert the string to the buffer 
// at point position 
void insert(string input, int position) 
{ 
    int len = input.length(); 
    int i = 0; 
  
    // If the point is not the gap check 
    // and move the cursor to that point 
    if (position != gap_left) { 
        move_cursor(position); 
    } 
  
    // Insert characters one by one 
    while (i < len) { 
        // If the gap is empty grow the size 
        if (gap_right == gap_left) { 
            int k = 10; 
            grow(k, position); 
        } 
  
        // Insert the character in the gap and 
        // move the gap 
        buffer[gap_left] = input[i]; 
        gap_left++; 
        i++; 
        position++;
    } 
} 
  
// Driver code 
int main() 
{ 
    // Initializing the gap buffer with size 10 
    for (int i = 0; i < 10; i++) { 
        buffer[i] = '_'; 
    } 
  
    cout << "Initializing the gap buffer "
         << "with size 10" << endl;
   
    for (int i = 0; i < size; i++) { 
        cout << buffer[i]<<" "; 
    } 
  
    cout << endl; 
  
    // Inserting a string to buffer 
    string input = "GEEKSGEEKS"; 
    int position = 0; 
  
    insert(input, position); 
  
    cout << endl; 
    cout << "Inserting a string to buffer"
         << ": GEEKSGEEKS" << endl; 
    cout << "Output: "; 
    for (int i = 0; i < size; i++) { 
        cout << buffer[i]<<" "; 
    } 
  
    input = "FOR"; 
    position = 5; 
  
    insert(input, position); 
  @y
    cout << endl; 
    cout << endl; 
      
    cout << "Inserting a string to buffer"
         << ": FOR" << endl; 
    cout << "Output: "; 
    for (int i = 0; i < size; i++) { 
        cout << buffer[i]<<" "; 
    } 
  
  
    return 0;
}
*/

//constants
const SIZE: usize = 512;

pub struct GapBuffer<> {
    size: usize,
    buffer: [u8; 512],
    gap_size: usize,
    gap_left: usize,
    gap_right: usize,
}

impl<> GapBuffer {
    //ellides: get_left<'a>(&'a self) -> &'a [u8]
    //concrete lifetime is most constraining ( the lifetime of the reference being used)
    //naturally, so long as ref is used, can't mutate gap buffer
    //ref will be free'd immediately after printing so it's ok.
    pub fn get_left(&self) -> &[u8] {
        &self.buffer[..self.gap_left]
    }

    pub fn get_right(&self) -> &[u8] {
        &self.buffer[self.gap_right+1..]
    }
    
    fn grow(&mut self, position: usize, k: usize) {
        let mut temp = [0; SIZE];

        for i in position..SIZE {
            temp[i - position] = self.buffer[i];
        }

        for i in 0..k {
            self.buffer[position + k + i] = temp[i];
        }

        self.size += k;
        self.gap_right += k;
    }

    fn left(&mut self, position: usize) {
        while position < self.gap_left {
            self.gap_left -= 1;
            self.gap_right -= 1;
            self.buffer[self.gap_right+1] = self.buffer[self.gap_left];
            self.buffer[self.gap_left]=0;
        }
    }

    fn right(&mut self, position: usize) {
        while position > self.gap_left {
            self.gap_left += 1;
            self.gap_right += 1;
            self.buffer[self.gap_left-1] = self.buffer[self.gap_right];
            self.buffer[self.gap_right]=0;
        }
    }

    fn move_cursor(&mut self, position: usize) {
        if position < self.gap_left {
            self.left(position);
        } else {
            self.right(position);
        }
    }
 
    pub fn deletion(&mut self, position: usize) {
        if position + 1 != self.gap_left {
            self.move_cursor(position + 1);
        }

        self.gap_left -= 1;
        self.buffer[self.gap_left] = 0;
    }
    
    //only supporting Ascii? ( u8 )
    pub fn insert(&mut self, input: &[u8], mut position: usize) {
        let len = input.len();
        let mut i = 0;

        if position != self.gap_left {
            self.move_cursor(position)
        }
        
        while i < len {
            if self.gap_right == self.gap_left {
                let k = 10;
                self.grow(k, position);
            }

            self.buffer[self.gap_left] = input[i];
            self.gap_left += 1;
            
            i += 1;
            position += 1;
        }
    }

    pub fn new(size: usize) -> Self {
        let mut gap_buffer = GapBuffer {
            //0 for now in case u wanna type in an underscore
            buffer: [0; SIZE],
            size,
            gap_size: 10,
            gap_left: 0,
            gap_right: size-1,
        };

        //init
        for i in 0..size { 
            gap_buffer.buffer[i] = 0; 
        } 

        gap_buffer
    }
}