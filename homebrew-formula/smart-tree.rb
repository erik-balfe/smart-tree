class SmartTree < Formula
  desc "A modern directory tree viewer with intelligent folding and display options"
  homepage "https://github.com/erik-balfe/smart-tree"
  url "https://github.com/erik-balfe/smart-tree/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "UPDATE_WITH_REAL_HASH_WHEN_AVAILABLE"
  license "MIT"
  head "https://github.com/erik-balfe/smart-tree.git", branch: "master"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Create a simple directory structure
    (testpath/"test_dir").mkpath
    (testpath/"test_dir/file1.txt").write("Hello")
    (testpath/"test_dir/file2.txt").write("World")
    (testpath/"test_dir/subdir").mkpath
    (testpath/"test_dir/subdir/file3.txt").write("!")

    # Run smart-tree on the test directory
    output = shell_output("#{bin}/smart-tree #{testpath}/test_dir")
    
    # Verify expected output
    assert_match "test_dir", output
    assert_match "file1.txt", output
    assert_match "file2.txt", output
    assert_match "subdir", output
  end
end