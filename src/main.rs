/* practice02 by rust(cargo)
 * 		written by Matsumoto Kazuki
 *
 * guess the number：数当てゲーム
 * 		1～100の中から数字がランダムに選ばれ，
 * 		標準入力からその数字をあてる(入力すると数字との大小関係を返す)
 * topic
 * console I/O, number, loop, rand
 *
 */

 extern crate rand;																				/* 外部クレート(ライブラリ)の使用
																									 * Cargo.tomlに依存関係を記述する必要がある
																									 * [dependencies]に入れる
																									 * クレート名 = "バージョン番号"
																									 * バージョン番号に互換性のある最新の
																									 * クレートをリンクする
																									 *
																									 * 例）
																									 * rand = "0.3.14"
																									 */
 use std::io;																						// includeのようにuseで示すことで一部
 use std::cmp::Ordering;																		// (std::など)省略できたりする
 use rand::Rng;																					//乱数生成器を示す．
																									//乱数を使う時にスコープに含める必要がある

 fn main()
 {
	println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1, 101);

	loop
	{
		println!("Please input your guess.");													/* println!("string");
																									 * stringを出力するマクロ
																									 * "!"はマクロを示す記号
																									 *
																									 */
		let mut guess = String::new();														/* let
																									 * 変数を生成する命令
																									 * Rustでは変数は標準const
																									 * "mut"を付すことで可変に変更する
																									 * 型は初期化する値(右辺)によって決定される
																									 */
																									/* String::new()
																									 * 空の文字列型を示す
																									 * 【文字列型】
																									 * 伸長可能、 可変、所有権のあるUTF-8エンコード
																									 *
																									 * 	固定文字で初期化したいときは
																									 * "constant string".to_string()
																									 * String::from("constant string")
																									 * とすればよい
																									 */
		io::stdin().read_line(&mut guess)														//参照先も常時constなので"mut"で可変化
					.expect("Failed to read line");												/* std::io::stdin().read(&mut buff)
																									 * -> use io; を入れると"std::"を省略できる
																									 * 標準入力をbuffにコピー
																									 *
																									 * "read"を"read_line"にすると読み込みを
																									 * 文字列に限定する(buffも文字列型が要求される)
																									 */
																									/* .expect("error message string")
																									 * Result型のメソッド
																									 * 返り値はOkかErrだが，
																									 * 呼び出し("."ピリオドで繋げて同時に使う時)では
																									 * Okの場合は同時に本体の関数が実行
																									 * Errの場合にはerror message stringを表示して
																									 * 強制終了する
																									 */
																									/* .parse()
																									 * 【Result型】
																									 * strに含まれるメソッド
																									 * 他の型へキャストする
																									 * atoiのように数字にも適用可能
																									 * Result型で帰ってくるので，その処理が必要
																									 *
																									 * 【str】：文字列スライス
																									 * 【String】と異なり，固定長文字列
																									 * charに似た感じ
																									 */
																									/* .trim()
																									 * 前後の空白を切り取るメソッド
																									 *
																									 */
		let guess: u32 = match guess.trim().parse()											//変数側に": type"を付けることで変数の型を明示できる
							{
								Ok(num) => num,
								Err(_) => continue,
							};
		println!("You guessed: {}", guess);
																									/* .cmp(&item)
																									 * 比較型のメソッド
																									 * 呼び出し元(ピリオドをつけたもの)と
																									 * itemを比較する
																									 * "::"によりmax，min，
																									 * Ordering::Less		呼び出し元がitemより小さい
																									 * Ordering::Greater	呼び出し元がitemより大きい
																									 * Ordering::Equal 		呼び出し元とitemが等しい
																									 * などのように使用できる
																									 */
		match guess.cmp(&secret_number)													// match item
		{																							// フロー演算子，switchの拡張版
			Ordering::Less 		=> println!("Too small!"),									// itemに符合する処理を実行するような分岐処理
			Ordering::Greater 	=> println!("Too big!"),
			Ordering::Equal 	=> {
											println!("You win!");
											break;
										}
		}
	}
}



