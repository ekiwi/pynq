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
  input  [72:0] io_in,
  output [72:0] io_out,
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
  reg [72:0] mem [0:63];
  reg [95:0] _RAND_1;
  wire [72:0] mem__T_38_data;
  wire [5:0] mem__T_38_addr;
  wire [72:0] mem__T_48_data;
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
  _RAND_1 = {3{$random}};
  `ifdef RANDOMIZE_MEM_INIT
  for (initvar = 0; initvar < 64; initvar = initvar+1)
    mem[initvar] = _RAND_1[72:0];
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
  wire  q_clock;
  wire  q_reset;
  wire [72:0] q_io_in;
  wire [72:0] q_io_out;
  wire  q_io_push_back;
  wire  q_io_pop_front;
  wire  q_io_full;
  wire  q_io_empty;
  wire [6:0] q_io_len;
  wire  _T_13;
  wire [8:0] _T_14;
  wire [72:0] _T_15;
  wire  _T_17;
  wire [63:0] _T_18;
  wire [7:0] _T_19;
  wire  _T_20;
  Queue q (
    .clock(q_clock),
    .reset(q_reset),
    .io_in(q_io_in),
    .io_out(q_io_out),
    .io_push_back(q_io_push_back),
    .io_pop_front(q_io_pop_front),
    .io_full(q_io_full),
    .io_empty(q_io_empty),
    .io_len(q_io_len)
  );
  assign _T_13 = q_io_full == 1'h0;
  assign _T_14 = {io_s_axis_tlast,io_s_axis_tkeep};
  assign _T_15 = {_T_14,io_s_axis_tdata};
  assign _T_17 = q_io_empty == 1'h0;
  assign _T_18 = q_io_out[63:0];
  assign _T_19 = q_io_out[71:64];
  assign _T_20 = q_io_out[72];
  assign io_s_axis_tready = _T_13;
  assign io_m_axis_tvalid = _T_17;
  assign io_m_axis_tdata = _T_18;
  assign io_m_axis_tkeep = _T_19;
  assign io_m_axis_tlast = _T_20;
  assign q_clock = clock;
  assign q_reset = reset;
  assign q_io_push_back = io_s_axis_tvalid;
  assign q_io_in = _T_15;
  assign q_io_pop_front = io_m_axis_tready;
endmodule
