// Version that compiles with std for demonstration
use std::println;

/// Simple DMA Buffer with exclusive ownership
pub struct DmaBuffer {
    data: Vec<u32>,  // Using Vec for simplicity in this demo
    buffer_id: u8,
}

impl DmaBuffer {
    pub fn new(size: usize, id: u8) -> Self {
        Self {
            data: vec![0; size],
            buffer_id: id,
        }
    }
    
    pub fn start_transfer(self) -> DmaTransfer {
        println!("Starting DMA transfer with buffer {}", self.buffer_id);
        DmaTransfer {
            buffer: self,
            is_complete: false,
        }
    }
    
    pub fn id(&self) -> u8 {
        self.buffer_id
    }
    
    pub fn write_data(&mut self, value: u32) {
        for item in self.data.iter_mut() {
            *item = value;
        }
    }
    
    pub fn read_data(&self) -> &[u32] {
        &self.data
    }
}

pub struct DmaTransfer {
    buffer: DmaBuffer,
    is_complete: bool,
}

impl DmaTransfer {
    pub fn complete(mut self) -> DmaBuffer {
        println!("DMA transfer completed for buffer {}", self.buffer.buffer_id);
        self.is_complete = true;
        self.buffer
    }
}

pub struct DmaManager {
    buffers: [Option<DmaBuffer>; 2],
}

impl DmaManager {
    pub fn new() -> Self {
        Self {
            buffers: [
                Some(DmaBuffer::new(4, 0)),
                Some(DmaBuffer::new(4, 1)),
            ],
        }
    }
    
    pub fn allocate_buffer(&mut self) -> Option<DmaBuffer> {
        for buffer_slot in &mut self.buffers {
            if buffer_slot.is_some() {
                return buffer_slot.take();
            }
        }
        None
    }
    
    pub fn free_buffer(&mut self, buffer: DmaBuffer) -> Result<(), &'static str> {
        let id = buffer.buffer_id as usize;
        
        if self.buffers[id].is_some() {
            return Err("Buffer slot already occupied!");
        }
        
        self.buffers[id] = Some(buffer);
        Ok(())
    }
    
    pub fn available_buffers(&self) -> usize {
        self.buffers.iter().filter(|b| b.is_some()).count()
    }
}

fn main() {
    // Create DMA manager
    let mut dma_manager = DmaManager::new();
    
    println!("=== Embedded DMA Ownership Example ===");
    println!("Available buffers: {}", dma_manager.available_buffers());
    
    // Demonstration 1: Basic ownership transfer
    println!("\n1. Basic ownership transfer:");
    
    let buffer = dma_manager.allocate_buffer().unwrap();
    println!("Allocated buffer {}", buffer.id());
    
    let transfer = buffer.start_transfer();
    // buffer is now moved - can't use it here
    
    let buffer = transfer.complete();
    println!("Got buffer {} back", buffer.id());
    
    dma_manager.free_buffer(buffer).unwrap();
    
    // Demonstration 2: Exclusive access
    println!("\n2. Exclusive access demonstration:");
    
    let buffer1 = dma_manager.allocate_buffer().unwrap();
    let buffer2 = dma_manager.allocate_buffer().unwrap();
    
    // Both buffers are independently owned
    println!("Have buffers {} and {}", buffer1.id(), buffer2.id());
    
    // Try to allocate third - fails
    let buffer3 = dma_manager.allocate_buffer();
    assert!(buffer3.is_none(), "Should not get third buffer");
    println!("Correctly rejected third buffer allocation");
    
    dma_manager.free_buffer(buffer1).unwrap();
    dma_manager.free_buffer(buffer2).unwrap();
    
    // Demonstration 3: Mutation safety
    println!("\n3. Mutation safety:");
    
    let mut buffer = dma_manager.allocate_buffer().unwrap();
    buffer.write_data(42);
    println!("Buffer data: {:?}", buffer.read_data());
    
    // Can't have another mutable reference while we have this one
    // This is enforced by the borrow checker
    
    dma_manager.free_buffer(buffer).unwrap();
    
    println!("\nAll demonstrations complete!");
    println!("Final available buffers: {}", dma_manager.available_buffers());
}