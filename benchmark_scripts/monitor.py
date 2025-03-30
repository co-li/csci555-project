#!/usr/bin/env python3
import argparse
import csv
import os
import psutil
import subprocess
import sys
import time
import socket
import threading
from datetime import datetime
from pathlib import Path


class ProcessMonitor:
    def __init__(self, process_pid, output_dir=None, interval=1.0, port=None):
        self.process_pid = process_pid
        self.interval = interval
        self.running = False
        self.process = None
        self.port = port
        self.server_thread = None
        
        if output_dir is None:
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            output_dir = f"monitoring_{timestamp}"
        
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(exist_ok=True, parents=True)
        
        self.csv_path = self.output_dir / "resource_usage.csv"
        self.csv_file = None
        self.csv_writer = None

    def start_monitoring(self):
        try:
            self.process = psutil.Process(self.process_pid)
        except psutil.NoSuchProcess:
            print(f"Error: Process with PID {self.process_pid} not found")
            return False
        
        self.csv_file = open(self.csv_path, 'w', newline='')
        self.csv_writer = csv.writer(self.csv_file)
        self.csv_writer.writerow([
            'Timestamp', 
            'CPU_Percent', 
            'Memory_RSS_MB',
            'Memory_Percent',
            'Threads',
            'IO_Read_MB',
            'IO_Write_MB'
        ])
        
        self.running = True
        
        # Start the socket server if a port is specified
        if self.port:
            self.server_thread = threading.Thread(target=self.start_socket_server)
            self.server_thread.daemon = True
            self.server_thread.start()
            print(f"Started monitoring process {self.process_pid}. Press Ctrl+C or send any message to port {self.port} to stop.")
        else:
            print(f"Started monitoring process {self.process_pid}. Press Ctrl+C to stop.")
            
        return True
        
    def start_socket_server(self):
        try:
            server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            server_socket.bind(('0.0.0.0', self.port))
            server_socket.settimeout(1.0)  # 1 second timeout for checking self.running
            server_socket.listen(1)
            
            print(f"Listening for termination messages on port {self.port}")
            
            while self.running:
                try:
                    client_socket, address = server_socket.accept()
                    print(f"\nReceived connection from {address}. Stopping monitoring.")
                    client_socket.close()
                    self.running = False
                except socket.timeout:
                    # This is just to periodically check if self.running is still True
                    continue
                except Exception as e:
                    print(f"\nError in socket server: {e}")
                    break
                    
            server_socket.close()
            
        except Exception as e:
            print(f"\nFailed to start socket server on port {self.port}: {e}")

    def monitor_loop(self):
        start_time = time.time()
        
        try:
            while self.running:
                if not self.process.is_running():
                    print(f"Process {self.process_pid} has terminated.")
                    break
                
                timestamp = time.time() - start_time
                
                try:
                    cpu_percent = self.process.cpu_percent(interval=0.1)
                    
                    memory_info = self.process.memory_info()
                    memory_rss_mb = memory_info.rss / (1024 * 1024)
                    memory_percent = self.process.memory_percent()
                    
                    thread_count = self.process.num_threads()
                    
                    # Try to get I/O counters, use dash if unavailable
                    io_read_value = "-"
                    io_write_value = "-"
                    io_available = False
                    
                    try:
                        io_counters = self.process.io_counters()
                        if hasattr(io_counters, 'read_bytes') and hasattr(io_counters, 'write_bytes'):
                            io_read_mb = io_counters.read_bytes / (1024 * 1024)
                            io_write_mb = io_counters.write_bytes / (1024 * 1024)
                            io_read_value = f"{io_read_mb:.2f}"
                            io_write_value = f"{io_write_mb:.2f}"
                            io_available = True
                    except (AttributeError, psutil.AccessDenied):
                        pass
                    
                    self.csv_writer.writerow([
                        f"{timestamp:.2f}",
                        f"{cpu_percent:.2f}",
                        f"{memory_rss_mb:.2f}",
                        f"{memory_percent:.2f}",
                        thread_count,
                        io_read_value,
                        io_write_value
                    ])
                    
                    io_info = " | I/O: not available" if not io_available else f" | I/O Read: {io_read_value} MB, Write: {io_write_value} MB"
                    print(f"\rRunning: {timestamp:.1f}s | CPU: {cpu_percent:.1f}% | Memory: {memory_rss_mb:.1f} MB{io_info}", end="")
                    
                except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess) as e:
                    print(f"\nError monitoring process: {e}")
                    break
                
                elapsed = time.time() - (start_time + timestamp)
                sleep_time = max(0, self.interval - elapsed)
                if sleep_time > 0:
                    time.sleep(sleep_time)
                    
        except KeyboardInterrupt:
            print("\nMonitoring stopped by user")
        finally:
            self.stop_monitoring()

    def stop_monitoring(self):
        self.running = False
        if self.csv_file:
            self.csv_file.close()
            print(f"\nData saved to {self.csv_path}")


def run_binary(binary_path, *args):
    try:
        cmd = [binary_path] + list(args)
        print(f"Running command: {' '.join(cmd)}")
        process = subprocess.Popen(
            cmd,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            text=True,
            bufsize=1
        )
        return process
    except Exception as e:
        print(f"Error running binary: {e}")
        return None
    
def print_subprocess_output(process):
    import threading
    
    def read_output(stream, prefix):
        for line in iter(stream.readline, ''):
            print(f"{prefix}: {line.rstrip()}")
    
    stdout_thread = threading.Thread(target=read_output, args=(process.stdout, "STDOUT"))
    stderr_thread = threading.Thread(target=read_output, args=(process.stderr, "STDERR"))
    
    # Set as daemon threads so they exit when the main thread exits
    stdout_thread.daemon = True
    stderr_thread.daemon = True

    stdout_thread.start()
    stderr_thread.start()


def main():
    parser = argparse.ArgumentParser(description='Monitor resource usage of a binary')
    parser.add_argument('binary_path', help='Path to the binary to monitor')
    parser.add_argument('--args', nargs='*', default=[], help='Arguments to pass to the binary')
    parser.add_argument('--interval', type=float, default=1.0, help='Monitoring interval in seconds')
    parser.add_argument('--output-dir', help='Directory to save monitoring data')
    parser.add_argument('--attach', type=int, help='Attach to existing process ID instead of launching new binary')
    parser.add_argument('--port', type=int, help='Port to listen for termination messages')
    
    args = parser.parse_args()
    
    if args.attach:
        process_pid = args.attach
        print(f"Attaching to existing process with PID: {process_pid}")
    else:
        if not os.path.isfile(args.binary_path):
            print(f"Error: Binary not found at {args.binary_path}")
            return 1
            
        process = run_binary(args.binary_path, *args.args)
        if not process:
            return 1
        process_pid = process.pid
        print(f"Started process with PID: {process_pid}")
    
    monitor = ProcessMonitor(process_pid, args.output_dir, args.interval, args.port)
    if not monitor.start_monitoring():
        return 1
    
    # print_subprocess_output(process)
    
    monitor.monitor_loop()
    
    if not args.attach and 'process' in locals() and process.poll() is None:
        print(f"Terminating the binary process (PID: {process_pid})")
        process.terminate()
        process.wait(timeout=5)
    
    return 0


if __name__ == "__main__":
    sys.exit(main())