`ifdef RANDOMIZE_GARBAGE_ASSIGN
`define RANDOMIZE
`endif
`ifdef RANDOMIZE_INVALID_ASSIGN
`define RANDOMIZE
`endif
`ifdef RANDOMIZE_REG_INIT
`define RANDOMIZE
`endif
`ifdef RANDOMIZE_MEM_INIT
`define RANDOMIZE
`endif

module Queue(
  input         clock,
  input         reset,
  input  [63:0] io_in,
  output [63:0] io_out,
  input         io_push_back,
  input         io_pop_front,
  output        io_full,
  output        io_empty,
  output [6:0]  io_len
);
  wire  _T_10;
  wire  _T_12;
  wire  _T_14;
  wire  do_push;
  wire  _T_16;
  wire  do_pop;
  reg [6:0] len;
  reg [31:0] _RAND_0;
  wire  _T_20;
  wire  _T_21;
  wire [7:0] _T_23;
  wire [6:0] _T_24;
  wire  _T_26;
  wire  _T_27;
  wire [7:0] _T_29;
  wire [7:0] _T_30;
  wire [6:0] _T_31;
  wire [6:0] _T_32;
  wire [6:0] _T_33;
  reg [63:0] mem [0:63];
  reg [63:0] _RAND_1;
  wire [63:0] mem__T_38_data;
  wire [5:0] mem__T_38_addr;
  wire [63:0] mem__T_48_data;
  wire [5:0] mem__T_48_addr;
  wire  mem__T_48_mask;
  wire  mem__T_48_en;
  reg [5:0] read_address;
  reg [31:0] _RAND_2;
  wire  _T_39;
  wire [6:0] _T_42;
  wire [5:0] _T_43;
  wire [5:0] _T_44;
  wire [5:0] _T_45;
  reg [5:0] write_address;
  reg [31:0] _RAND_3;
  wire  _T_49;
  wire [6:0] _T_52;
  wire [5:0] _T_53;
  wire [5:0] _T_54;
  wire [5:0] _T_55;
  assign _T_10 = io_len == 7'h40;
  assign _T_12 = io_len == 7'h0;
  assign _T_14 = io_full == 1'h0;
  assign do_push = io_push_back & _T_14;
  assign _T_16 = io_empty == 1'h0;
  assign do_pop = io_pop_front & _T_16;
  assign _T_20 = do_pop == 1'h0;
  assign _T_21 = do_push & _T_20;
  assign _T_23 = len + 7'h1;
  assign _T_24 = _T_23[6:0];
  assign _T_26 = do_push == 1'h0;
  assign _T_27 = do_pop & _T_26;
  assign _T_29 = len - 7'h1;
  assign _T_30 = $unsigned(_T_29);
  assign _T_31 = _T_30[6:0];
  assign _T_32 = _T_27 ? _T_31 : len;
  assign _T_33 = _T_21 ? _T_24 : _T_32;
  assign mem__T_38_addr = read_address;
  assign mem__T_38_data = mem[mem__T_38_addr];
  assign mem__T_48_data = io_in;
  assign mem__T_48_addr = write_address;
  assign mem__T_48_mask = do_push;
  assign mem__T_48_en = do_push;
  assign _T_39 = read_address == 6'h3f;
  assign _T_42 = read_address + 6'h1;
  assign _T_43 = _T_42[5:0];
  assign _T_44 = _T_39 ? 6'h0 : _T_43;
  assign _T_45 = do_pop ? _T_44 : read_address;
  assign _T_49 = write_address == 6'h3f;
  assign _T_52 = write_address + 6'h1;
  assign _T_53 = _T_52[5:0];
  assign _T_54 = _T_49 ? 6'h0 : _T_53;
  assign _T_55 = do_push ? _T_54 : write_address;
  assign io_out = mem__T_38_data;
  assign io_full = _T_10;
  assign io_empty = _T_12;
  assign io_len = len;
`ifdef RANDOMIZE
  integer initvar;
  initial begin
    `ifndef verilator
      #0.002 begin end
    `endif
  `ifdef RANDOMIZE_REG_INIT
  _RAND_0 = {1{$random}};
  len = _RAND_0[6:0];
  `endif // RANDOMIZE_REG_INIT
  _RAND_1 = {2{$random}};
  `ifdef RANDOMIZE_MEM_INIT
  for (initvar = 0; initvar < 64; initvar = initvar+1)
    mem[initvar] = _RAND_1[63:0];
  `endif // RANDOMIZE_MEM_INIT
  `ifdef RANDOMIZE_REG_INIT
  _RAND_2 = {1{$random}};
  read_address = _RAND_2[5:0];
  `endif // RANDOMIZE_REG_INIT
  `ifdef RANDOMIZE_REG_INIT
  _RAND_3 = {1{$random}};
  write_address = _RAND_3[5:0];
  `endif // RANDOMIZE_REG_INIT
  end
`endif // RANDOMIZE
  always @(posedge clock) begin
    if (reset) begin
      len <= 7'h0;
    end else begin
      if (_T_21) begin
        len <= _T_24;
      end else begin
        if (_T_27) begin
          len <= _T_31;
        end
      end
    end
    if(mem__T_48_en & mem__T_48_mask) begin
      mem[mem__T_48_addr] <= mem__T_48_data;
    end
    if (reset) begin
      read_address <= 6'h0;
    end else begin
      if (do_pop) begin
        if (_T_39) begin
          read_address <= 6'h0;
        end else begin
          read_address <= _T_43;
        end
      end
    end
    if (reset) begin
      write_address <= 6'h0;
    end else begin
      if (do_push) begin
        if (_T_49) begin
          write_address <= 6'h0;
        end else begin
          write_address <= _T_53;
        end
      end
    end
  end
endmodule
module Queue_1(
  input        clock,
  input        reset,
  input  [7:0] io_in,
  output [7:0] io_out,
  input        io_push_back,
  input        io_pop_front,
  output       io_full,
  output       io_empty,
  output [6:0] io_len
);
  wire  _T_10;
  wire  _T_12;
  wire  _T_14;
  wire  do_push;
  wire  _T_16;
  wire  do_pop;
  reg [6:0] len;
  reg [31:0] _RAND_0;
  wire  _T_20;
  wire  _T_21;
  wire [7:0] _T_23;
  wire [6:0] _T_24;
  wire  _T_26;
  wire  _T_27;
  wire [7:0] _T_29;
  wire [7:0] _T_30;
  wire [6:0] _T_31;
  wire [6:0] _T_32;
  wire [6:0] _T_33;
  reg [7:0] mem [0:63];
  reg [31:0] _RAND_1;
  wire [7:0] mem__T_38_data;
  wire [5:0] mem__T_38_addr;
  wire [7:0] mem__T_48_data;
  wire [5:0] mem__T_48_addr;
  wire  mem__T_48_mask;
  wire  mem__T_48_en;
  reg [5:0] read_address;
  reg [31:0] _RAND_2;
  wire  _T_39;
  wire [6:0] _T_42;
  wire [5:0] _T_43;
  wire [5:0] _T_44;
  wire [5:0] _T_45;
  reg [5:0] write_address;
  reg [31:0] _RAND_3;
  wire  _T_49;
  wire [6:0] _T_52;
  wire [5:0] _T_53;
  wire [5:0] _T_54;
  wire [5:0] _T_55;
  assign _T_10 = io_len == 7'h40;
  assign _T_12 = io_len == 7'h0;
  assign _T_14 = io_full == 1'h0;
  assign do_push = io_push_back & _T_14;
  assign _T_16 = io_empty == 1'h0;
  assign do_pop = io_pop_front & _T_16;
  assign _T_20 = do_pop == 1'h0;
  assign _T_21 = do_push & _T_20;
  assign _T_23 = len + 7'h1;
  assign _T_24 = _T_23[6:0];
  assign _T_26 = do_push == 1'h0;
  assign _T_27 = do_pop & _T_26;
  assign _T_29 = len - 7'h1;
  assign _T_30 = $unsigned(_T_29);
  assign _T_31 = _T_30[6:0];
  assign _T_32 = _T_27 ? _T_31 : len;
  assign _T_33 = _T_21 ? _T_24 : _T_32;
  assign mem__T_38_addr = read_address;
  assign mem__T_38_data = mem[mem__T_38_addr];
  assign mem__T_48_data = io_in;
  assign mem__T_48_addr = write_address;
  assign mem__T_48_mask = do_push;
  assign mem__T_48_en = do_push;
  assign _T_39 = read_address == 6'h3f;
  assign _T_42 = read_address + 6'h1;
  assign _T_43 = _T_42[5:0];
  assign _T_44 = _T_39 ? 6'h0 : _T_43;
  assign _T_45 = do_pop ? _T_44 : read_address;
  assign _T_49 = write_address == 6'h3f;
  assign _T_52 = write_address + 6'h1;
  assign _T_53 = _T_52[5:0];
  assign _T_54 = _T_49 ? 6'h0 : _T_53;
  assign _T_55 = do_push ? _T_54 : write_address;
  assign io_out = mem__T_38_data;
  assign io_full = _T_10;
  assign io_empty = _T_12;
  assign io_len = len;
`ifdef RANDOMIZE
  integer initvar;
  initial begin
    `ifndef verilator
      #0.002 begin end
    `endif
  `ifdef RANDOMIZE_REG_INIT
  _RAND_0 = {1{$random}};
  len = _RAND_0[6:0];
  `endif // RANDOMIZE_REG_INIT
  _RAND_1 = {1{$random}};
  `ifdef RANDOMIZE_MEM_INIT
  for (initvar = 0; initvar < 64; initvar = initvar+1)
    mem[initvar] = _RAND_1[7:0];
  `endif // RANDOMIZE_MEM_INIT
  `ifdef RANDOMIZE_REG_INIT
  _RAND_2 = {1{$random}};
  read_address = _RAND_2[5:0];
  `endif // RANDOMIZE_REG_INIT
  `ifdef RANDOMIZE_REG_INIT
  _RAND_3 = {1{$random}};
  write_address = _RAND_3[5:0];
  `endif // RANDOMIZE_REG_INIT
  end
`endif // RANDOMIZE
  always @(posedge clock) begin
    if (reset) begin
      len <= 7'h0;
    end else begin
      if (_T_21) begin
        len <= _T_24;
      end else begin
        if (_T_27) begin
          len <= _T_31;
        end
      end
    end
    if(mem__T_48_en & mem__T_48_mask) begin
      mem[mem__T_48_addr] <= mem__T_48_data;
    end
    if (reset) begin
      read_address <= 6'h0;
    end else begin
      if (do_pop) begin
        if (_T_39) begin
          read_address <= 6'h0;
        end else begin
          read_address <= _T_43;
        end
      end
    end
    if (reset) begin
      write_address <= 6'h0;
    end else begin
      if (do_push) begin
        if (_T_49) begin
          write_address <= 6'h0;
        end else begin
          write_address <= _T_53;
        end
      end
    end
  end
endmodule
module AxisQueue(
  input         clock,
  input         reset,
  input         io_s_axis_tvalid,
  output        io_s_axis_tready,
  input  [63:0] io_s_axis_tdata,
  input  [7:0]  io_s_axis_tkeep,
  input         io_s_axis_tlast,
  output        io_m_axis_tvalid,
  input         io_m_axis_tready,
  output [63:0] io_m_axis_tdata,
  output [7:0]  io_m_axis_tkeep,
  output        io_m_axis_tlast
);
  wire  data_clock;
  wire  data_reset;
  wire [63:0] data_io_in;
  wire [63:0] data_io_out;
  wire  data_io_push_back;
  wire  data_io_pop_front;
  wire  data_io_full;
  wire  data_io_empty;
  wire [6:0] data_io_len;
  wire  keep_clock;
  wire  keep_reset;
  wire [7:0] keep_io_in;
  wire [7:0] keep_io_out;
  wire  keep_io_push_back;
  wire  keep_io_pop_front;
  wire  keep_io_full;
  wire  keep_io_empty;
  wire [6:0] keep_io_len;
  wire  _T_14;
  wire  _T_16;
  wire  _T_17;
  wire  _T_19;
  wire  _T_21;
  wire  _T_22;
  Queue data (
    .clock(data_clock),
    .reset(data_reset),
    .io_in(data_io_in),
    .io_out(data_io_out),
    .io_push_back(data_io_push_back),
    .io_pop_front(data_io_pop_front),
    .io_full(data_io_full),
    .io_empty(data_io_empty),
    .io_len(data_io_len)
  );
  Queue_1 keep (
    .clock(keep_clock),
    .reset(keep_reset),
    .io_in(keep_io_in),
    .io_out(keep_io_out),
    .io_push_back(keep_io_push_back),
    .io_pop_front(keep_io_pop_front),
    .io_full(keep_io_full),
    .io_empty(keep_io_empty),
    .io_len(keep_io_len)
  );
  assign _T_14 = data_io_full == 1'h0;
  assign _T_16 = keep_io_full == 1'h0;
  assign _T_17 = _T_14 & _T_16;
  assign _T_19 = data_io_empty == 1'h0;
  assign _T_21 = keep_io_empty == 1'h0;
  assign _T_22 = _T_19 & _T_21;
  assign io_s_axis_tready = _T_17;
  assign io_m_axis_tvalid = _T_22;
  assign io_m_axis_tdata = data_io_out;
  assign io_m_axis_tkeep = keep_io_out;
  assign io_m_axis_tlast = 1'h0;
  assign data_clock = clock;
  assign data_reset = reset;
  assign keep_clock = clock;
  assign keep_reset = reset;
  assign data_io_push_back = io_s_axis_tvalid;
  assign keep_io_push_back = io_s_axis_tvalid;
  assign data_io_in = io_s_axis_tdata;
  assign keep_io_in = io_s_axis_tkeep;
  assign data_io_pop_front = io_m_axis_tready;
  assign keep_io_pop_front = io_m_axis_tready;
endmodule
